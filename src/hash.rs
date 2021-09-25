use crate::{Block, Error, Result, SignerError, VerifierError};
use openssl::pkey::{HasPublic, PKey, PKeyRef, Private};
use openssl::{hash::MessageDigest, rsa::Rsa, sha::Sha256, sign::Signer, sign::Verifier};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hash([u8; 32]);

impl Hash {
    const RSA_BITS: u32 = 2048;

    fn gen_keypair() -> Result<PKey<Private>> {
        let keypair = Rsa::generate(Self::RSA_BITS).map_err(Error::KeyGen)?;
        let keypair = PKey::from_rsa(keypair).map_err(Error::KeyGen)?;
        Ok(keypair)
    }
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
    ) -> Result<(Self, Vec<u8>, PKey<Private>)> {
        Self::new_existing_keypair(previous, data, Self::gen_keypair()?)
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
        let mut verifier = Verifier::new(msgd(), pkey).map_err(VerifierError::Create)?;
        verifier
            .update(data.as_ref())
            .map_err(VerifierError::Update)?;

        let signature_verified = verifier
            .verify(signature.as_ref())
            .map_err(VerifierError::Execute)?;
        if !signature_verified {
            return Ok(false);
        }

        Ok(self.0 == hash_triplet(previous.into(), signature, data))
    }

    /// Creates a new hash from the previous one alongside hte core data included
    /// within the hash, manually inputting the public/private keypair.
    fn new_existing_keypair(
        previous: impl Into<&'a Hash>,
        data: impl AsRef<[u8]>,
        keypair: PKey<Private>,
    ) -> Result<(Self, Vec<u8>, PKey<Private>)> {
        let keypair_signer = keypair.clone();

        let mut signer = Signer::new(msgd(), &keypair_signer).map_err(SignerError::Create)?;
        signer.update(data.as_ref()).map_err(SignerError::Update)?;

        let signature = signer.sign_to_vec().map_err(SignerError::Execute)?;

        Ok((
            Self(hash_triplet(previous.into(), signature.as_slice(), data)),
            signature,
            keypair,
        ))
    }
}

impl Default for Hash {
    /// Creates default genesis hash.
    fn default() -> Self {
        // null forbids babes to feed dead bad beef to dudes
        Self([
            0x0, 0x4, 0xB, 0x1, 0xD, 0xB, 0xA, 0xB, 0xE, 0x5, 0x2, 0xF, 0xE, 0xE, 0xD, 0xD, 0xE,
            0xA, 0xD, 0xB, 0xA, 0xD, 0xB, 0xE, 0xE, 0xF, 0x2, 0xD, 0x0, 0x0, 0xD, 0x5,
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

fn msgd() -> MessageDigest {
    MessageDigest::sha256()
}

fn hash_triplet(previous: &Hash, signature: impl AsRef<[u8]>, data: impl AsRef<[u8]>) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(&previous.0[..]);
    hasher.update(signature.as_ref());
    hasher.update(data.as_ref());
    hasher.finish()
}
