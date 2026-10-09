use crate::domain::{hash, CTX_SHORT_CODE};
use crate::Error;
use p256::ecdsa::VerifyingKey;
use std::fmt;

/// A P-256 public key in 33-byte compressed SEC1 form. Construction validates the point.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct PubKey([u8; 33]);

impl PubKey {
    /// Accepts compressed or uncompressed SEC1 and stores the compressed form.
    pub fn from_sec1(bytes: &[u8]) -> Result<Self, Error> {
        let vk = VerifyingKey::from_sec1_bytes(bytes).map_err(|_| Error::BadKey)?;
        Ok(Self::from_verifying_key(&vk))
    }

    pub fn from_verifying_key(vk: &VerifyingKey) -> Self {
        let point = vk.to_encoded_point(true);
        let mut out = [0u8; 33];
        out.copy_from_slice(point.as_bytes());
        PubKey(out)
    }

    pub fn as_bytes(&self) -> &[u8; 33] {
        &self.0
    }

    pub(crate) fn verifying_key(&self) -> Result<VerifyingKey, Error> {
        VerifyingKey::from_sec1_bytes(&self.0).map_err(|_| Error::BadKey)
    }

    /// `did:key` form: base58btc of multicodec p256-pub (0x1200, varint `80 24`) + key.
    pub fn did_key(&self) -> String {
        let mut buf = vec![0x80, 0x24];
        buf.extend(self.0);
        format!("did:key:z{}", bs58::encode(buf).into_string())
    }
}

impl fmt::Debug for PubKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.did_key())
    }
}

/// A self-certifying Bucks ID: UUID with version nibble 8 and RFC 9562 variant.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BucksId([u8; 16]);

const CROCKFORD: &[u8; 32] = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";

impl BucksId {
    pub(crate) fn from_digest(digest: &[u8; 32]) -> Self {
        let mut b = [0u8; 16];
        b.copy_from_slice(&digest[..16]);
        b[6] = (b[6] & 0x0f) | 0x80;
        b[8] = (b[8] & 0x3f) | 0x80;
        BucksId(b)
    }

    /// Interpret 16 bytes as a Bucks ID, checking version and variant bits.
    pub fn from_bytes(b: [u8; 16]) -> Result<Self, Error> {
        if b[6] >> 4 != 8 || b[8] >> 6 != 2 {
            return Err(Error::BadId);
        }
        Ok(BucksId(b))
    }

    pub fn as_bytes(&self) -> &[u8; 16] {
        &self.0
    }

    pub fn did(&self) -> String {
        format!("did:bucks:{self}")
    }

    /// Eight Crockford base32 characters from the first 40 bits of a domain-separated hash.
    /// A label for humans, not an identifier: it is not unique.
    pub fn short_code(&self) -> String {
        let h = hash(CTX_SHORT_CODE, &self.0);
        let n = h[..5].iter().fold(0u64, |n, x| (n << 8) | *x as u64);
        (0..8)
            .rev()
            .map(|i| CROCKFORD[((n >> (i * 5)) & 31) as usize] as char)
            .collect()
    }

    /// Parse the text form, with or without the `did:bucks:` prefix. Lowercase or uppercase hex.
    pub fn parse(s: &str) -> Result<Self, Error> {
        let s = s.strip_prefix("did:bucks:").unwrap_or(s);
        let bytes = s.as_bytes();
        if bytes.len() != 36 || [8, 13, 18, 23].iter().any(|&i| bytes[i] != b'-') {
            return Err(Error::BadId);
        }
        let hex: Vec<u8> = bytes.iter().copied().filter(|c| *c != b'-').collect();
        // from_str_radix accepts a leading '+', so check the characters explicitly.
        if hex.len() != 32 || !hex.iter().all(u8::is_ascii_hexdigit) {
            return Err(Error::BadId);
        }
        let mut b = [0u8; 16];
        for (i, out) in b.iter_mut().enumerate() {
            let pair = std::str::from_utf8(&hex[2 * i..2 * i + 2]).map_err(|_| Error::BadId)?;
            *out = u8::from_str_radix(pair, 16).map_err(|_| Error::BadId)?;
        }
        Self::from_bytes(b)
    }
}

impl fmt::Display for BucksId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, x) in self.0.iter().enumerate() {
            if matches!(i, 4 | 6 | 8 | 10) {
                f.write_str("-")?;
            }
            write!(f, "{x:02x}")?;
        }
        Ok(())
    }
}

impl fmt::Debug for BucksId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self}")
    }
}
