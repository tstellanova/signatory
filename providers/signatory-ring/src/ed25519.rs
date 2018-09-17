//! Ed25519 signer and verifier implementation for *ring*

use ring;
use ring::signature::Ed25519KeyPair;
use untrusted;

use signatory::{
    ed25519::{Ed25519PublicKey, Ed25519Signature, FromSeed, Seed},
    encoding::FromPkcs8,
    error::{Error, ErrorKind},
    PublicKeyed, Signature, Signer, Verifier,
};

/// Ed25519 signature provider for *ring*
pub struct Ed25519Signer(Ed25519KeyPair);

impl FromSeed for Ed25519Signer {
    /// Create a new Ed25519Signer from an unexpanded seed value
    fn from_seed<S: Into<Seed>>(seed: S) -> Self {
        let keypair =
            Ed25519KeyPair::from_seed_unchecked(untrusted::Input::from(&seed.into().0[..]))
                .unwrap();

        Ed25519Signer(keypair)
    }
}

impl FromPkcs8 for Ed25519Signer {
    /// Create a new Ed25519Signer from a PKCS#8 encoded private key
    fn from_pkcs8(pkcs8_bytes: &[u8]) -> Result<Self, Error> {
        let keypair = Ed25519KeyPair::from_pkcs8(untrusted::Input::from(pkcs8_bytes))
            .map_err(|_| Error::from(ErrorKind::KeyInvalid))?;

        Ok(Ed25519Signer(keypair))
    }
}

impl PublicKeyed<Ed25519PublicKey> for Ed25519Signer {
    fn public_key(&self) -> Result<Ed25519PublicKey, Error> {
        Ok(Ed25519PublicKey::from_bytes(self.0.public_key_bytes()).unwrap())
    }
}

impl Signer<Ed25519Signature> for Ed25519Signer {
    fn sign(&self, msg: &[u8]) -> Result<Ed25519Signature, Error> {
        Ok(Ed25519Signature::from_bytes(self.0.sign(msg).as_ref()).unwrap())
    }
}

/// Ed25519 verifier for *ring*
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Ed25519Verifier(Ed25519PublicKey);

impl<'a> From<&'a Ed25519PublicKey> for Ed25519Verifier {
    fn from(public_key: &'a Ed25519PublicKey) -> Self {
        Ed25519Verifier(*public_key)
    }
}

impl Verifier<Ed25519Signature> for Ed25519Verifier {
    fn verify(&self, msg: &[u8], signature: &Ed25519Signature) -> Result<(), Error> {
        ring::signature::verify(
            &ring::signature::ED25519,
            untrusted::Input::from(self.0.as_bytes()),
            untrusted::Input::from(msg),
            untrusted::Input::from(signature.as_bytes()),
        ).map_err(|_| ErrorKind::SignatureInvalid.into())
    }
}

#[cfg(test)]
mod tests {
    use super::{Ed25519Signer, Ed25519Verifier};
    ed25519_tests!(Ed25519Signer, Ed25519Verifier);
}
