//! D-33 exact borrowed schemas. No secret plaintext is serialized here.
use super::{Error, KdfParameters};
use crate::foundation::{
    Error as CborError,
    cbor::{self, Bytes, Class, Decode, Encode, Reader, Text, Writer},
};

pub(super) const MAX_BUNDLE_BYTES: usize = 32768;
#[derive(Clone, Copy)]
pub(super) struct Kdf<'a> {
    pub algorithm: &'a str,
    pub version: u64,
    pub memory_kib: u64,
    pub time_cost: u64,
    pub parallelism: u64,
    pub output_len: u64,
    pub salt: &'a [u8],
}
impl Encode for Kdf<'_> {
    fn encode(&self, w: &mut Writer) -> Result<(), CborError> {
        w.map([
            ("algorithm", &Text(self.algorithm) as &dyn Encode),
            ("version", &self.version),
            ("memory_kib", &self.memory_kib),
            ("time_cost", &self.time_cost),
            ("parallelism", &self.parallelism),
            ("output_len", &self.output_len),
            ("salt", &Bytes(self.salt)),
        ])
    }
}
impl<'a> Decode<'a> for Kdf<'a> {
    fn decode(r: &mut Reader<'a, '_>) -> Result<Self, CborError> {
        let mut v = Self {
            algorithm: "",
            version: 0,
            memory_kib: 0,
            time_cost: 0,
            parallelism: 0,
            output_len: 0,
            salt: &[],
        };
        // Exact count + seen-field checks ensure every field is assigned once.
        r.map(
            [
                "algorithm",
                "version",
                "memory_kib",
                "time_cost",
                "parallelism",
                "output_len",
                "salt",
            ],
            |r, i| {
                match i {
                    0 => v.algorithm = r.text()?,
                    1 => v.version = r.uint()?,
                    2 => v.memory_kib = r.uint()?,
                    3 => v.time_cost = r.uint()?,
                    4 => v.parallelism = r.uint()?,
                    5 => v.output_len = r.uint()?,
                    6 => v.salt = r.bytes()?,
                    _ => return Err(CborError::Schema),
                };
                Ok(())
            },
        )?;
        Ok(v)
    }
}
#[derive(Clone, Copy)]
pub(super) struct Header<'a> {
    pub schema_version: u64,
    pub recipient_id: &'a [u8],
    pub kem_algorithm: &'a str,
    pub kem_key_id: &'a [u8],
    pub signature_algorithm: &'a str,
    pub signature_key_id: &'a [u8],
    pub aead_algorithm: &'a str,
    pub kdf: Kdf<'a>,
}
impl Header<'_> {
    pub fn validate(&self) -> Result<KdfParameters, Error> {
        if self.schema_version != 1 {
            return Err(Error::UnsupportedVersion);
        }
        if self.kem_algorithm != "ML-KEM-768"
            || self.signature_algorithm != "ML-DSA-65"
            || self.aead_algorithm != "AES-256-GCM"
            || self.kdf.algorithm != "Argon2id"
        {
            return Err(Error::UnsupportedAlgorithm);
        }
        if self.recipient_id.len() != 32
            || self.kem_key_id.len() != 32
            || self.signature_key_id.len() != 32
        {
            return Err(Error::InvalidBundle);
        }
        if self.kdf.version != 19 || self.kdf.output_len != 32 || self.kdf.salt.len() != 16 {
            return Err(Error::InvalidKdfParameters);
        }
        KdfParameters::new(
            self.kdf.memory_kib,
            self.kdf.time_cost,
            self.kdf.parallelism,
        )
    }
}
impl Encode for Header<'_> {
    fn encode(&self, w: &mut Writer) -> Result<(), CborError> {
        w.map([
            ("schema_version", &self.schema_version as &dyn Encode),
            ("recipient_id", &Bytes(self.recipient_id)),
            ("kem_algorithm", &Text(self.kem_algorithm)),
            ("kem_key_id", &Bytes(self.kem_key_id)),
            ("signature_algorithm", &Text(self.signature_algorithm)),
            ("signature_key_id", &Bytes(self.signature_key_id)),
            ("aead_algorithm", &Text(self.aead_algorithm)),
            ("kdf", &self.kdf),
        ])
    }
}
impl<'a> Decode<'a> for Header<'a> {
    fn decode(r: &mut Reader<'a, '_>) -> Result<Self, CborError> {
        let mut v = Self {
            schema_version: 0,
            recipient_id: &[],
            kem_algorithm: "",
            kem_key_id: &[],
            signature_algorithm: "",
            signature_key_id: &[],
            aead_algorithm: "",
            kdf: Kdf {
                algorithm: "",
                version: 0,
                memory_kib: 0,
                time_cost: 0,
                parallelism: 0,
                output_len: 0,
                salt: &[],
            },
        };
        r.map(
            [
                "schema_version",
                "recipient_id",
                "kem_algorithm",
                "kem_key_id",
                "signature_algorithm",
                "signature_key_id",
                "aead_algorithm",
                "kdf",
            ],
            |r, i| {
                match i {
                    0 => v.schema_version = r.uint()?,
                    1 => v.recipient_id = r.bytes()?,
                    2 => v.kem_algorithm = r.text()?,
                    3 => v.kem_key_id = r.bytes()?,
                    4 => v.signature_algorithm = r.text()?,
                    5 => v.signature_key_id = r.bytes()?,
                    6 => v.aead_algorithm = r.text()?,
                    7 => v.kdf = Kdf::decode(r)?,
                    _ => return Err(CborError::Schema),
                };
                Ok(())
            },
        )?;
        Ok(v)
    }
}
#[derive(Clone, Copy)]
pub(super) struct Protected<'a> {
    pub nonce: &'a [u8],
    pub ciphertext_and_tag: &'a [u8],
}
impl Encode for Protected<'_> {
    fn encode(&self, w: &mut Writer) -> Result<(), CborError> {
        w.map([
            ("nonce", &Bytes(self.nonce) as &dyn Encode),
            ("ciphertext_and_tag", &Bytes(self.ciphertext_and_tag)),
        ])
    }
}
impl<'a> Decode<'a> for Protected<'a> {
    fn decode(r: &mut Reader<'a, '_>) -> Result<Self, CborError> {
        let mut v = Self {
            nonce: &[],
            ciphertext_and_tag: &[],
        };
        r.map(["nonce", "ciphertext_and_tag"], |r, i| {
            match i {
                0 => v.nonce = r.bytes()?,
                1 => v.ciphertext_and_tag = r.bytes()?,
                _ => return Err(CborError::Schema),
            };
            Ok(())
        })?;
        Ok(v)
    }
}
#[derive(Clone, Copy)]
pub(super) struct Bundle<'a> {
    pub header: Header<'a>,
    pub kem: Protected<'a>,
    pub signature: Protected<'a>,
}
impl Encode for Bundle<'_> {
    fn encode(&self, w: &mut Writer) -> Result<(), CborError> {
        w.map([
            ("header", &self.header as &dyn Encode),
            ("kem", &self.kem),
            ("signature", &self.signature),
        ])
    }
}
impl<'a> Decode<'a> for Bundle<'a> {
    fn decode(r: &mut Reader<'a, '_>) -> Result<Self, CborError> {
        let mut h = None;
        let mut k = None;
        let mut s = None;
        r.map(["header", "kem", "signature"], |r, i| {
            match i {
                0 => h = Some(Header::decode(r)?),
                1 => k = Some(Protected::decode(r)?),
                2 => s = Some(Protected::decode(r)?),
                _ => return Err(CborError::Schema),
            };
            Ok(())
        })?;
        Ok(Self {
            header: h.ok_or(CborError::Schema)?,
            kem: k.ok_or(CborError::Schema)?,
            signature: s.ok_or(CborError::Schema)?,
        })
    }
}
pub(super) fn parse(bytes: &[u8]) -> Result<Bundle<'_>, Error> {
    let v: Bundle<'_> = cbor::validated(bytes, Class::Bundle)?;
    v.header.validate()?;
    for entry in [v.kem, v.signature] {
        // Only the two bounded v1 encrypted lengths may reach authentication.
        // Authenticate cross-role swaps rather than treating their size as proof
        // of role. Exact role-specific plaintext length is checked after AEAD;
        // no swapped/wrong-length key can be returned as an unlocked key.
        if entry.nonce.len() != 12 || ![2416, 4048].contains(&entry.ciphertext_and_tag.len()) {
            return Err(Error::InvalidBundle);
        }
    }
    if v.kem.nonce == v.signature.nonce {
        return Err(Error::InvalidBundle);
    }
    Ok(v)
}
pub(super) fn encode(bundle: &Bundle<'_>) -> Result<Vec<u8>, Error> {
    cbor::deterministic(bundle, Class::Bundle).map_err(|e| match e {
        CborError::AllocationFailure => Error::AllocationFailure,
        _ => Error::SerializationFailure,
    })
}
#[derive(Clone, Copy)]
pub(super) enum Role {
    Kem,
    Signature,
}
struct Aad<'a, 'b> {
    header: &'a Header<'b>,
    role: Role,
}
impl Encode for Aad<'_, '_> {
    fn encode(&self, w: &mut Writer) -> Result<(), CborError> {
        w.map([
            ("header", self.header as &dyn Encode),
            (
                "role",
                &Text(match self.role {
                    Role::Kem => "kem",
                    Role::Signature => "signature",
                }),
            ),
        ])
    }
}
pub(super) fn aad(header: &Header<'_>, role: Role) -> Result<Vec<u8>, Error> {
    use cbor::{Domain, Separated};
    cbor::deterministic(
        &Separated {
            domain: Domain::KeyBundleAad,
            body: &Aad { header, role },
        },
        Class::Bundle,
    )
    .map_err(|_| Error::SerializationFailure)
}
