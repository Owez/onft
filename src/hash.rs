use crate::{Block, Error, Result, SignerError, VerifierError};
use openssl::pkey::{HasPublic, PKey, PKeyRef, Private};
use openssl::{sha::Sha256, sign::Signer, sign::Verifier};

/// Hash of a single block, containing the previous hash along with the message
/// signature and the data
///
/// # Example
///
/// TODO: example
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hash([u8; 32]);

impl Hash {
    /// Length of ED25518-based signatures in bytes
    pub const SIG_LEN: usize = 64;
}

impl<'a> Hash {
    /// Creates a new hash from the previous one alongside the core data included
    /// within the hash, automatically generating the public/private keypair;
    /// returning this hash, the signature and the aforementioned keypair.
    ///
    /// # Example
    ///
    /// TODO: example
    pub fn new(
        previous: impl Into<&'a Hash>,
        data: impl AsRef<[u8]>,
    ) -> Result<(Self, [u8; Self::SIG_LEN], PKey<Private>)> {
        Self::new_existing_keypair(previous, data, gen_keypair()?)
    }

    /// Verifies current hash using it's known `signature`, the `pkey` public key
    /// and `data` whilst using the `previous` hash.
    ///
    /// # Example
    ///
    /// TODO: example
    pub fn verify(
        &self,
        previous: impl Into<&'a Hash>,
        signature: impl AsRef<[u8]>,
        data: impl AsRef<[u8]>,
        pkey: &PKeyRef<impl HasPublic>,
    ) -> Result<bool> {
        let mut verifier = Verifier::new_without_digest(pkey).map_err(VerifierError::Create)?;
        let signature_verified = verifier
            .verify_oneshot(signature.as_ref(), data.as_ref())
            .map_err(VerifierError::Execute)?;

        Ok(if signature_verified {
            self.0 == hash_triplet(previous.into(), signature, data)
        } else {
            false
        })
    }

    /// Creates a new hash from the previous one alongside hte core data included
    /// within the hash, manually inputting the public/private keypair.
    fn new_existing_keypair(
        previous: impl Into<&'a Hash>,
        data: impl AsRef<[u8]>,
        keypair: PKey<Private>,
    ) -> Result<(Self, [u8; Self::SIG_LEN], PKey<Private>)> {
        let keypair_signer = keypair.clone();

        let mut signer =
            Signer::new_without_digest(&keypair_signer).map_err(SignerError::Create)?;

        let mut signature = [0; Self::SIG_LEN];
        signer
            .sign_oneshot(&mut signature, data.as_ref())
            .map_err(SignerError::Update)?;

        Ok((
            Self(hash_triplet(previous.into(), signature, data)),
            signature,
            keypair,
        ))
    }
}

impl Default for Hash {
    /// Creates default genesis hash.
    fn default() -> Self {
        Self([
            66, 108, 111, 111, 100, 121, 32, 103, 101, 110, 101, 115, 105, 115, 32, 98, 108, 111,
            99, 107, 32, 109, 101, 115, 115, 97, 103, 101, 115, 46, 46, 46,
        ])
    }
}

impl From<Block> for Hash {
    fn from(block: Block) -> Self {
        block.hash
    }
}

impl<'a> From<&'a Block> for &'a Hash {
    fn from(block: &'a Block) -> Self {
        &block.hash
    }
}

fn gen_keypair() -> Result<PKey<Private>> {
    PKey::generate_ed25519().map_err(Error::KeyGen)
}

fn hash_triplet(previous: &Hash, signature: impl AsRef<[u8]>, data: impl AsRef<[u8]>) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(&previous.0[..]);
    hasher.update(signature.as_ref());
    hasher.update(data.as_ref());
    hasher.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gen_keypair() {
        super::gen_keypair().unwrap();
    }

    #[test]
    fn create_verify_hash() {
        const DATA: &str = "Hello, world!";
        let (hash, signature, pkey) = Hash::new(&Hash::default(), DATA.as_bytes()).unwrap();
        let verified = hash
            .verify(&Hash::default(), signature, DATA, &pkey)
            .unwrap();

        if !verified {
            panic!("Valid hash not verified successfully")
        }
    }
}
