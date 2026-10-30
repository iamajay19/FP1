//! Linux safe-std persistence of ciphertext only. Private, current-user-owned
//! directories and trusted ancestors are required. This does not defend against
//! concurrent path replacement by root/the same trusted UID; no openat sandbox.
use super::{
    EncryptedBundle, Error, Identity, Password, UnlockedKeys, random, schema, unlock_bundle,
};
use std::sync::Mutex;
static PERSISTENCE: Mutex<()> = Mutex::new(());
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::{MetadataExt, OpenOptionsExt};
use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::{Component, Path, PathBuf},
};

#[derive(Debug, Clone, Copy)]
pub enum Persistence {
    CreateNew,
    Replace,
}
fn io(_: std::io::Error) -> Error {
    Error::IoFailure
}
fn uid() -> Result<u32, Error> {
    // Safe Linux metadata API, avoiding a new project FFI/unsafe boundary.
    fs::metadata("/proc/self").map(|m| m.uid()).map_err(io)
}
fn private_file(m: &fs::Metadata, owner: u32) -> bool {
    m.is_file() && m.uid() == owner && m.mode() & 0o077 == 0
}
fn checked_parent(path: &Path) -> Result<(PathBuf, PathBuf, File, u32), Error> {
    if path.as_os_str().as_bytes().len() > 4096 {
        return Err(Error::UnsafePath);
    }
    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir().map_err(io)?.join(path)
    };
    let name = absolute.file_name().ok_or(Error::UnsafePath)?.to_owned();
    let parent = absolute.parent().ok_or(Error::UnsafePath)?;
    let owner = uid()?;
    let root_owner = fs::metadata("/").map_err(io)?.uid();
    let mut walked = PathBuf::new();
    for component in parent.components() {
        match component {
            Component::RootDir | Component::Normal(_) => walked.push(component.as_os_str()),
            Component::CurDir => continue,
            _ => return Err(Error::UnsafePath),
        }
        let m = fs::symlink_metadata(&walked).map_err(io)?;
        // Managed user namespaces can report the root filesystem owner as an
        // unmapped UID. Trust only the literal / and root-owned sticky /tmp
        // anchors in that case, never arbitrary unmapped-user-owned ancestors.
        let root_anchor = walked == Path::new("/");
        let tmp_anchor =
            walked == Path::new("/tmp") && m.uid() == root_owner && m.mode() & 0o1000 != 0;
        if !m.is_dir() || (!root_anchor && !tmp_anchor && m.uid() != owner && m.uid() != 0) {
            return Err(Error::UnsafePath);
        }
        // Root-owned sticky /tmp is allowed; its user's private child is checked
        // below. Writable non-sticky or other-user ancestors reject.
        if m.mode() & 0o022 != 0 && !((m.uid() == 0 || tmp_anchor) && m.mode() & 0o1000 != 0) {
            return Err(Error::UnsafePath);
        }
    }
    let m = fs::symlink_metadata(parent).map_err(io)?;
    if m.uid() != owner || m.mode() & 0o077 != 0 {
        return Err(Error::UnsafePath);
    }
    let directory = File::open(parent).map_err(io)?;
    same_directory(parent, &directory)?;
    Ok((parent.to_path_buf(), parent.join(name), directory, owner))
}
fn same_directory(path: &Path, file: &File) -> Result<(), Error> {
    let a = fs::symlink_metadata(path).map_err(io)?;
    let b = file.metadata().map_err(io)?;
    if !a.is_dir() || a.dev() != b.dev() || a.ino() != b.ino() {
        return Err(Error::UnsafePath);
    }
    Ok(())
}
fn read_checked(path: &Path, owner: u32) -> Result<Vec<u8>, Error> {
    let before = fs::symlink_metadata(path).map_err(io)?;
    if !private_file(&before, owner) || before.len() > schema::MAX_BUNDLE_BYTES as u64 {
        return Err(Error::UnsafePath);
    }
    let mut file = File::open(path).map_err(io)?;
    let after = file.metadata().map_err(io)?;
    if !private_file(&after, owner) || before.dev() != after.dev() || before.ino() != after.ino() {
        return Err(Error::UnsafePath);
    }
    let mut bytes = Vec::new();
    bytes
        .try_reserve_exact(schema::MAX_BUNDLE_BYTES + 1)
        .map_err(|_| Error::AllocationFailure)?;
    // Fixed fallible allocation, no read_to_end growth/OOM path driven by a
    // file that expands while being read. One extra byte detects oversize.
    bytes.resize(schema::MAX_BUNDLE_BYTES + 1, 0);
    let mut used = 0usize;
    loop {
        let n = file.read(&mut bytes[used..]).map_err(io)?;
        if n == 0 {
            break;
        }
        used = used.checked_add(n).ok_or(Error::InvalidBundle)?;
        if used > schema::MAX_BUNDLE_BYTES {
            return Err(Error::InvalidBundle);
        }
    }
    bytes.truncate(used);
    Ok(bytes)
}
struct Temporary(PathBuf);
impl Drop for Temporary {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.0);
    }
}
fn temporary(parent: &Path) -> Result<(Temporary, File), Error> {
    for _ in 0..4 {
        let mut nonce = [0u8; 16];
        random(&mut nonce)?;
        let mut suffix = String::new();
        suffix
            .try_reserve_exact(32)
            .map_err(|_| Error::AllocationFailure)?;
        const HEX: &[u8; 16] = b"0123456789abcdef";
        for v in nonce {
            suffix.push(char::from(HEX[usize::from(v >> 4)]));
            suffix.push(char::from(HEX[usize::from(v & 15)]));
        }
        let path = parent.join(format!(".cadp-key-{suffix}.tmp"));
        match OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o600)
            .open(&path)
        {
            Ok(file) => return Ok((Temporary(path), file)),
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(e) => return Err(io(e)),
        }
    }
    Err(Error::RandomnessFailure)
}
/// Atomically publish already-encrypted canonical bytes. Replace requires the
/// existing valid header binding and fresh salt/nonces, but does not unlock it.
/// There is no plaintext/password-changing command or backup/recovery mechanism.
pub fn persist(path: &Path, bundle: &EncryptedBundle, policy: Persistence) -> Result<(), Error> {
    let _transaction = PERSISTENCE.lock().map_err(|_| Error::IoFailure)?;
    let incoming = schema::parse(bundle.as_bytes())?;
    let (parent, destination, directory, owner) = checked_parent(path)?;
    let existing = match fs::symlink_metadata(&destination) {
        Ok(m) => {
            if !private_file(&m, owner) {
                return Err(Error::UnsafePath);
            }
            match policy {
                Persistence::CreateNew => return Err(Error::AlreadyExists),
                Persistence::Replace => Some(read_checked(&destination, owner)?),
            }
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            if matches!(policy, Persistence::Replace) {
                return Err(Error::IoFailure);
            }
            None
        }
        Err(e) => return Err(io(e)),
    };
    if let Some(old) = &existing {
        let previous = schema::parse(old)?;
        if previous.header.recipient_id != incoming.header.recipient_id
            || previous.header.kem_key_id != incoming.header.kem_key_id
            || previous.header.signature_key_id != incoming.header.signature_key_id
        {
            return Err(Error::InvalidBundle);
        }
        if previous.header.kdf.salt == incoming.header.kdf.salt
            || [previous.kem.nonce, previous.signature.nonce]
                .iter()
                .any(|n| *n == incoming.kem.nonce || *n == incoming.signature.nonce)
        {
            return Err(Error::InvalidBundle);
        }
    }
    let (temporary, mut file) = temporary(&parent)?;
    #[cfg(test)]
    if super::tests::partial_write_fault() {
        file.write_all(&bundle.as_bytes()[..32]).map_err(io)?;
        return Err(Error::IoFailure);
    }
    file.write_all(bundle.as_bytes()).map_err(io)?;
    file.flush().map_err(io)?;
    file.sync_all().map_err(io)?;
    same_directory(&parent, &directory)?;
    match policy {
        Persistence::CreateNew => {
            // link is an atomic no-clobber publication on the same filesystem.
            fs::hard_link(&temporary.0, &destination).map_err(|e| {
                if e.kind() == std::io::ErrorKind::AlreadyExists {
                    Error::AlreadyExists
                } else {
                    io(e)
                }
            })?;
            fs::remove_file(&temporary.0).map_err(|_| Error::PersistenceUncertain)?;
        }
        Persistence::Replace => {
            // Recheck expected encrypted bytes before replacing. The private-dir
            // trust assumption still applies to races after this check.
            if existing.as_deref() != Some(read_checked(&destination, owner)?.as_slice()) {
                return Err(Error::UnsafePath);
            }
            fs::rename(&temporary.0, &destination).map_err(io)?;
        }
    }
    directory
        .sync_all()
        .map_err(|_| Error::PersistenceUncertain)?;
    Ok(())
}
pub fn unlock_file(
    path: &Path,
    password: &Password,
    identity: &Identity<'_>,
) -> Result<UnlockedKeys, Error> {
    let (parent, destination, directory, owner) = checked_parent(path)?;
    let bytes = read_checked(&destination, owner)?;
    same_directory(&parent, &directory)?;
    unlock_bundle(&bytes, password, identity)
}
