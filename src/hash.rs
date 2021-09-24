use crate::{Block, Error, Result, SignerError, VerifierError};
use openssl::pkey::{PKey, Private, Public};
use openssl::{hash::MessageDigest, rsa::Rsa, sha::Sha256, sign::Signer, sign::Verifier};

#[derive(Clone)]
pub struct Hash([u8; 32]);

impl Hash {
    pub const RSA_BITS: u32 = 2048;
    pub const GENESIS: Self = Hash([0; 32]);

    fn gen_keypair() -> Result<PKey<Private>> {
        let keypair = Rsa::generate(Self::RSA_BITS).map_err(Error::KeyGen)?;
        let keypair = PKey::from_rsa(keypair).map_err(Error::KeyGen)?;
        Ok(keypair)
    }
}

impl<'a> Hash {
    pub fn new(
        previous: impl Into<&'a Hash>,
        data: impl AsRef<[u8]>,
    ) -> Result<(Self, Vec<u8>, PKey<Private>)> {
        Self::new_existing_keypair(previous, data, Self::gen_keypair()?)
    }

    pub fn verify(
        &self,
        previous: impl Into<&'a Hash>,
        signature: impl AsRef<[u8]>,
        data: impl AsRef<[u8]>,
        pkey: PKey<Public>,
    ) -> Result<bool> {
        let mut verifier = Verifier::new(msgd(), &pkey).map_err(VerifierError::Create)?;
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
    fn default() -> Self {
        todo!()
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
