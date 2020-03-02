//! Traits for public keys

use core::fmt::Debug;
use signature::Error;

/// Signers which know their public keys (to be implemented by Signatory
/// providers)
pub trait PublicKeyed<K: PublicKey>: Send + Sync {
    /// Public key which can verify signatures created by this signer
    fn public_key(&self, key_image: KeyImage) -> Result<K, Error>;
}

/// Enum to allow selecting if a caller wants the Compressed or Uncompressed form of a public key. Only Uncompressed only implemented for secp256k1
pub enum KeyImage {
    /// A field element value + 1 byte
    Compressed,
    /// Two field elements
    Uncompressed,
}

/// Common trait for all public keys
pub trait PublicKey: AsRef<[u8]> + Debug + Sized + Eq + Ord {}
