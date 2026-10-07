//! Low-level minicbor is private. Only a typed Decode+Encode implementation may
//! produce an accepted object, and only after whole-original-byte comparison.
//! Structural scanning borrows input, uses O(depth) stack and never builds a Value tree.
use super::Error;
use minicbor::{Decoder, Encoder, encode::Write};

#[derive(Clone, Copy)]
pub(crate) enum Class {
    Small,
    Bundle,
    Crl,
}
impl Class {
    fn bytes(self) -> usize {
        match self {
            Self::Small => 16 * 1024,
            Self::Bundle => 32 * 1024,
            Self::Crl => 16 * 1024 * 1024,
        }
    }
    fn items(self) -> usize {
        match self {
            Self::Small | Self::Bundle => 256,
            Self::Crl => 1_000_000,
        }
    }
}
pub(crate) const MAX_DEPTH: usize = 8;
pub(crate) const MAX_CRL_ENTRIES: usize = 100_000;

struct Budget {
    remaining: usize,
}
impl Budget {
    fn charge(&mut self) -> Result<(), Error> {
        self.remaining = self.remaining.checked_sub(1).ok_or(Error::ItemLimit)?;
        Ok(())
    }
    fn children(&self, count: u64) -> Result<(), Error> {
        let count = usize::try_from(count).map_err(|_| Error::ItemLimit)?;
        if count > self.remaining {
            return Err(Error::ItemLimit);
        }
        Ok(())
    }
}

// Only the profile's five major types. In particular, the general codec's skip,
// indefinite iterators, tags, floats and generic collection decoders are not exposed.
fn scan(d: &mut Decoder<'_>, budget: &mut Budget, depth: usize) -> Result<(), Error> {
    if depth > MAX_DEPTH {
        return Err(Error::DepthLimit);
    }
    budget.charge()?;
    let start = d.position();
    let first = *d.input().get(start).ok_or(Error::MalformedCbor)?;
    let major = first >> 5;
    if first & 31 == 31 {
        return Err(Error::ForbiddenCbor);
    }
    match major {
        0 => {
            d.u64().map_err(|_| Error::MalformedCbor)?;
        }
        2 => {
            d.bytes().map_err(|_| Error::MalformedCbor)?;
        }
        3 => {
            d.str().map_err(|_| Error::MalformedCbor)?;
        }
        4 | 5 => {
            let count = if major == 4 { d.array() } else { d.map() }
                .map_err(|_| Error::MalformedCbor)?
                .ok_or(Error::ForbiddenCbor)?;
            let children = if major == 5 {
                count.checked_mul(2).ok_or(Error::ItemLimit)?
            } else {
                count
            };
            budget.children(children)?; // Before loops, conversions or allocations.
            if major == 4 && count > MAX_CRL_ENTRIES as u64 {
                return Err(Error::SizeLimit);
            }
            let mut previous: Option<&[u8]> = None;
            for _ in 0..count {
                if major == 5 {
                    let key_start = d.position();
                    if d.input().get(key_start).map(|v| v >> 5) != Some(3) {
                        return Err(Error::Schema);
                    }
                    scan(d, budget, depth + 1)?;
                    let key = d
                        .input()
                        .get(key_start..d.position())
                        .ok_or(Error::MalformedCbor)?;
                    if let Some(prev) = previous {
                        if prev == key {
                            return Err(Error::DuplicateField);
                        }
                        if prev > key {
                            return Err(Error::NonCanonical);
                        }
                    }
                    previous = Some(key);
                }
                scan(d, budget, depth + 1)?;
            }
        }
        _ => return Err(Error::ForbiddenCbor),
    }
    Ok(())
}

fn preflight(bytes: &[u8], class: Class, budget: &mut Budget, depth: usize) -> Result<(), Error> {
    if bytes.len() > class.bytes() {
        return Err(Error::SizeLimit);
    }
    let mut d = Decoder::new(bytes);
    scan(&mut d, budget, depth)?;
    if d.position() != bytes.len() {
        return Err(Error::TrailingBytes);
    }
    Ok(())
}

pub(crate) trait Encode {
    fn encode(&self, w: &mut Writer) -> Result<(), Error>;
}
pub(crate) trait Decode<'a>: Encode + Sized {
    fn decode(r: &mut Reader<'a, '_>) -> Result<Self, Error>;
}

pub(crate) struct Bytes<'a>(pub &'a [u8]);
pub(crate) struct Text<'a>(pub &'a str);
impl Encode for u64 {
    fn encode(&self, w: &mut Writer) -> Result<(), Error> {
        w.uint(*self)
    }
}
impl Encode for Bytes<'_> {
    fn encode(&self, w: &mut Writer) -> Result<(), Error> {
        w.bytes(self.0)
    }
}
impl Encode for Text<'_> {
    fn encode(&self, w: &mut Writer) -> Result<(), Error> {
        w.text(self.0)
    }
}

struct BoundedOutput {
    bytes: Vec<u8>,
    max: usize,
}
impl Write for BoundedOutput {
    type Error = Error;
    fn write_all(&mut self, bytes: &[u8]) -> Result<(), Error> {
        let length = self
            .bytes
            .len()
            .checked_add(bytes.len())
            .ok_or(Error::SizeLimit)?;
        if length > self.max {
            return Err(Error::SizeLimit);
        }
        if length > self.bytes.capacity() {
            // Geometric growth avoids quadratic re-encoding cost on large CRLs;
            // no reservation exceeds the class byte limit.
            let doubled = match self.bytes.capacity().checked_mul(2) {
                Some(n) => n,
                None => self.max,
            };
            let capacity = length.max(doubled).max(64).min(self.max);
            self.bytes
                .try_reserve_exact(capacity - self.bytes.len())
                .map_err(|_| Error::AllocationFailure)?;
        }
        self.bytes.extend_from_slice(bytes);
        Ok(())
    }
}
pub(crate) struct Writer {
    e: Encoder<BoundedOutput>,
}
impl Writer {
    pub(crate) fn uint(&mut self, v: u64) -> Result<(), Error> {
        self.e.u64(v).map_err(encode_error)?;
        Ok(())
    }
    pub(crate) fn bytes(&mut self, v: &[u8]) -> Result<(), Error> {
        self.e.bytes(v).map_err(encode_error)?;
        Ok(())
    }
    pub(crate) fn text(&mut self, v: &str) -> Result<(), Error> {
        self.e.str(v).map_err(encode_error)?;
        Ok(())
    }
    pub(crate) fn array(&mut self, len: u64) -> Result<(), Error> {
        self.e.array(len).map_err(encode_error)?;
        Ok(())
    }
    /// Keys are trusted schema constants, not attacker-sized strings. Sort actual
    /// encoded keys, not Rust lexical strings or the alternate length-first profile.
    pub(crate) fn map<const N: usize>(
        &mut self,
        fields: [(&str, &dyn Encode); N],
    ) -> Result<(), Error> {
        if N > 64 {
            return Err(Error::Schema);
        }
        let mut keys = [[0u8; 137]; N];
        let mut lengths = [0usize; N];
        let mut order = std::array::from_fn::<_, N, _>(|i| i);
        for (i, (key, _)) in fields.iter().enumerate() {
            if key.len() > 128 {
                return Err(Error::Schema);
            }
            let mut buffer = &mut keys[i][..];
            Encoder::new(&mut buffer)
                .str(key)
                .map_err(|_| Error::Schema)?;
            lengths[i] = 137 - buffer.len();
        }
        order.sort_unstable_by(|a, b| keys[*a][..lengths[*a]].cmp(&keys[*b][..lengths[*b]]));
        for pair in order.windows(2) {
            if fields[pair[0]].0 == fields[pair[1]].0 {
                return Err(Error::DuplicateField);
            }
        }
        self.e
            .map(u64::try_from(N).map_err(|_| Error::Schema)?)
            .map_err(encode_error)?;
        for i in order {
            self.text(fields[i].0)?;
            fields[i].1.encode(self)?;
        }
        Ok(())
    }
}
fn encode_error(e: minicbor::encode::Error<Error>) -> Error {
    match e.into_write() {
        Some(e) => e,
        None => Error::Schema,
    }
}

pub(crate) fn deterministic(value: &impl Encode, class: Class) -> Result<Vec<u8>, Error> {
    let mut w = Writer {
        e: Encoder::new(BoundedOutput {
            bytes: Vec::new(),
            max: class.bytes(),
        }),
    };
    value.encode(&mut w)?;
    let bytes = w.e.into_writer().bytes;
    preflight(
        &bytes,
        class,
        &mut Budget {
            remaining: class.items(),
        },
        1,
    )?;
    Ok(bytes)
}

pub(crate) struct Reader<'a, 'b> {
    d: Decoder<'a>,
    budget: &'b mut Budget,
    depth: usize,
}
impl<'a> Reader<'a, '_> {
    pub(crate) fn uint(&mut self) -> Result<u64, Error> {
        self.d.u64().map_err(|_| Error::Schema)
    }
    pub(crate) fn bytes(&mut self) -> Result<&'a [u8], Error> {
        self.d.bytes().map_err(|_| Error::Schema)
    }
    pub(crate) fn text(&mut self) -> Result<&'a str, Error> {
        self.d.str().map_err(|_| Error::Schema)
    }
    pub(crate) fn array<T>(
        &mut self,
        count: u64,
        f: impl FnOnce(&mut Self) -> Result<T, Error>,
    ) -> Result<T, Error> {
        if self.d.array().map_err(|_| Error::Schema)? != Some(count) {
            return Err(Error::Schema);
        }
        self.depth = self.depth.checked_add(1).ok_or(Error::DepthLimit)?;
        let result = f(self);
        self.depth -= 1;
        result
    }
    pub(crate) fn map<const N: usize>(
        &mut self,
        fields: [&str; N],
        mut f: impl FnMut(&mut Self, usize) -> Result<(), Error>,
    ) -> Result<(), Error> {
        if N > 64
            || self.d.map().map_err(|_| Error::Schema)?
                != Some(u64::try_from(N).map_err(|_| Error::Schema)?)
        {
            return Err(Error::Schema);
        }
        let mut seen = 0u64;
        self.depth = self.depth.checked_add(1).ok_or(Error::DepthLimit)?;
        let result = (|| {
            for _ in 0..N {
                let key = self.text()?;
                let index = fields
                    .iter()
                    .position(|v| *v == key)
                    .ok_or(Error::UnknownField)?;
                let mask = 1u64
                    .checked_shl(u32::try_from(index).map_err(|_| Error::Schema)?)
                    .ok_or(Error::Schema)?;
                if seen & mask != 0 {
                    return Err(Error::DuplicateField);
                }
                seen |= mask;
                f(self, index)?;
            }
            Ok(())
        })();
        self.depth -= 1;
        result
    }
    /// Embedded signed bytes inherit the same aggregate budget and nesting depth;
    /// never restart a fresh 256-item allowance per embedded object.
    pub(crate) fn embedded<T: Decode<'a>>(&mut self, class: Class) -> Result<T, Error> {
        let bytes = self.bytes()?;
        validated_with_budget(
            bytes,
            class,
            self.budget,
            self.depth.checked_add(1).ok_or(Error::DepthLimit)?,
        )
    }
}

pub(crate) fn validated<'a, T: Decode<'a>>(bytes: &'a [u8], class: Class) -> Result<T, Error> {
    validated_with_budget(
        bytes,
        class,
        &mut Budget {
            remaining: class.items(),
        },
        1,
    )
}
fn validated_with_budget<'a, T: Decode<'a>>(
    bytes: &'a [u8],
    class: Class,
    budget: &mut Budget,
    depth: usize,
) -> Result<T, Error> {
    preflight(bytes, class, budget, depth)?;
    let mut r = Reader {
        d: Decoder::new(bytes),
        budget,
        depth,
    };
    let value = T::decode(&mut r)?;
    if r.d.position() != bytes.len() {
        return Err(Error::TrailingBytes);
    }
    // Includes every header/length and the whole object. Overwide integer/string
    // encodings cannot be normalized into a signed object accepted by the caller.
    if deterministic(&value, class)?.as_slice() != bytes {
        return Err(Error::NonCanonical);
    }
    Ok(value)
}

/// Fixed protocol domains only; callers cannot supply arbitrary labels.
#[derive(Clone, Copy)]
pub(crate) enum Domain {
    KemId,
    SignatureId,
    Certificate,
    Crl,
    PopChallenge,
    PopKemKey,
    PopKem,
    PopSignature,
}
impl Domain {
    fn label(self) -> &'static str {
        match self {
            Self::KemId => "CADP-KEM-ID-v1",
            Self::SignatureId => "CADP-SIG-ID-v1",
            Self::Certificate => "CADP-CERT-v1",
            Self::Crl => "CADP-CRL-v1",
            Self::PopChallenge => "CADP-POP-CHALLENGE-v1",
            Self::PopKemKey => "CADP-POP-KEM-KEY-v1",
            Self::PopKem => "CADP-POP-KEM-v1",
            Self::PopSignature => "CADP-POP-SIG-v1",
        }
    }
}
pub(crate) struct Separated<'a, T: Encode> {
    pub domain: Domain,
    pub body: &'a T,
}
impl<T: Encode> Encode for Separated<'_, T> {
    fn encode(&self, w: &mut Writer) -> Result<(), Error> {
        w.array(2)?;
        w.text(self.domain.label())?;
        self.body.encode(w)
    }
}

#[cfg(test)]
mod boundary_tests {
    use super::*;
    #[test]
    fn p2a_cbor_010_structural_and_embedded_limits() {
        let mut allowed = vec![0x81; 7];
        allowed.push(0);
        assert!(preflight(&allowed, Class::Small, &mut Budget { remaining: 256 }, 1).is_ok());
        let mut forbidden = vec![0x81; 8];
        forbidden.push(0);
        assert_eq!(
            preflight(&forbidden, Class::Small, &mut Budget { remaining: 256 }, 1),
            Err(Error::DepthLimit)
        );
        // Embedded roots count below their byte-string slot, never at depth 1.
        assert_eq!(
            preflight(&[0x81, 0], Class::Small, &mut Budget { remaining: 256 }, 8),
            Err(Error::DepthLimit)
        );
        let mut out = BoundedOutput {
            bytes: Vec::new(),
            max: 16 * 1024,
        };
        let mut growth = 0;
        for _ in 0..16000 {
            let before = out.bytes.capacity();
            out.write_all(&[0]).unwrap();
            if before != out.bytes.capacity() {
                growth += 1;
            }
            assert!(out.bytes.capacity() <= out.max);
        }
        assert!(growth <= 9);
        assert_eq!(out.write_all(&[0; 385]), Err(Error::SizeLimit));
        assert_eq!(out.bytes.len(), 16000);
        // One-million-item budget; every container/key/value is charged.
        let mut bytes = vec![0x9a, 0, 1, 0x86, 0xa0]; // 100000 entries
        for _ in 0..99_999 {
            bytes.extend_from_slice(&[0x88, 0, 0, 0, 0, 0, 0, 0, 0]);
        }
        bytes.extend_from_slice(&[0x88, 0, 0, 0, 0, 0, 0, 0, 0]);
        let mut budget = Budget {
            remaining: 1_000_000,
        };
        assert!(preflight(&bytes, Class::Crl, &mut budget, 1).is_ok());
        assert_eq!(budget.remaining, 99_999); // 1 + 100000*(1+8)
        assert_eq!(
            preflight(
                &[0x9a, 0, 1, 0x86, 0xa1],
                Class::Crl,
                &mut Budget {
                    remaining: 1_000_000
                },
                1
            ),
            Err(Error::SizeLimit)
        );
        assert_eq!(
            preflight(&bytes, Class::Crl, &mut Budget { remaining: 900_000 }, 1),
            Err(Error::ItemLimit)
        );
    }
}
