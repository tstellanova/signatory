//! Ed25519 public keys

use core::fmt::{self, Debug};

#[cfg(all(feature = "alloc", feature = "encoding"))]
use encoding::Encode;
#[cfg(feature = "encoding")]
use encoding::{Decode, Encoding};
use error::Error;
#[allow(unused_imports)]
use prelude::*;
use util::fmt_colon_delimited_hex;
use PublicKey as PublicKeyTrait;

/// Size of an Ed25519 public key in bytes (256-bits)
pub const PUBLIC_KEY_SIZE: usize = 32;

/// Ed25519 public keys
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct PublicKey(pub [u8; PUBLIC_KEY_SIZE]);

impl PublicKey {
    /// Create an Ed25519 public key from a 32-byte array
    pub fn new(bytes: [u8; PUBLIC_KEY_SIZE]) -> Self {
        PublicKey(bytes)
    }

    /// Create an Ed25519 public key from its serialized (compressed Edwards-y) form
    pub fn from_bytes<B>(bytes: B) -> Result<Self, Error>
    where
        B: AsRef<[u8]>,
    {
        ensure!(
            bytes.as_ref().len() == PUBLIC_KEY_SIZE,
            KeyInvalid,
            "expected {}-byte key (got {})",
            PUBLIC_KEY_SIZE,
            bytes.as_ref().len()
        );

        let mut public_key = [0u8; PUBLIC_KEY_SIZE];
        public_key.copy_from_slice(bytes.as_ref());
        Ok(PublicKey(public_key))
    }

    /// Obtain public key as a byte array reference
    #[inline]
    pub fn as_bytes(&self) -> &[u8; PUBLIC_KEY_SIZE] {
        &self.0
    }

    /// Convert public key into owned byte array
    #[inline]
    pub fn into_bytes(self) -> [u8; PUBLIC_KEY_SIZE] {
        self.0
    }
}

impl AsRef<[u8]> for PublicKey {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl Debug for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "signatory::ed25519::PublicKey(")?;
        fmt_colon_delimited_hex(f, self.as_ref())?;
        write!(f, ")")
    }
}

#[cfg(feature = "encoding")]
impl Decode for PublicKey {
    /// Decode an Ed25519 seed from a byte slice with the given encoding (e.g. hex, Base64)
    fn decode(encoded_key: &[u8], encoding: Encoding) -> Result<Self, Error> {
        let mut decoded_key = [0u8; PUBLIC_KEY_SIZE];
        let decoded_len = encoding.decode(encoded_key, &mut decoded_key)?;

        ensure!(
            decoded_len == PUBLIC_KEY_SIZE,
            KeyInvalid,
            "invalid {}-byte public key (expected {})",
            decoded_len,
            PUBLIC_KEY_SIZE
        );

        Ok(Self::new(decoded_key))
    }
}

#[cfg(all(feature = "encoding", feature = "alloc"))]
impl Encode for PublicKey {
    /// Encode an Ed25519 seed with the given encoding (e.g. hex, Base64)
    fn encode(&self, encoding: Encoding) -> Vec<u8> {
        encoding.encode_vec(self.as_bytes())
    }
}

impl PublicKeyTrait for PublicKey {}
