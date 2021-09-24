use crate::{Error, Result, SignerError, VerifierError};
use openssl::pkey::{PKey, Private, Public};
use openssl::sign::Verifier;
use openssl::{hash::MessageDigest, rsa::Rsa, sha::Sha256, sign::Signer};

#[derive(Clone)]
pub struct Hash([u8; 32]);

impl Hash {
    pub const RSA_BITS: u32 = 2048;
    pub const GENESIS: Self = Hash([0; 32]);

    pub fn new(previous: &Hash, data: impl AsRef<[u8]>) -> Result<(Self, Vec<u8>, PKey<Private>)> {
        Self::new_existing_keypair(previous, data, Self::gen_keypair()?)
    }

    pub fn verify(
        &self,
        previous: &Hash,
        signature: impl AsRef<[u8]>,
        data: impl AsRef<[u8]>,
        pkey: PKey<Public>,
    ) -> Result<bool> {
        let mut verifier =
            Verifier::new(msgd(), &pkey).map_err(|err| VerifierError::Create(err))?;
        verifier
            .update(data.as_ref())
            .map_err(|err| VerifierError::Update(err))?;

        let signature_verified = verifier
            .verify(signature.as_ref())
            .map_err(|err| VerifierError::Execute(err))?;
        if !signature_verified {
            return Ok(false);
        }

        Ok(self.0 == hash_triplet(previous, signature, data))
    }

    fn new_existing_keypair(
        previous: &Hash,
        data: impl AsRef<[u8]>,
        keypair: PKey<Private>,
    ) -> Result<(Self, Vec<u8>, PKey<Private>)> {
        let keypair_signer = keypair.clone();

        let mut signer =
            Signer::new(msgd(), &keypair_signer).map_err(|err| SignerError::Create(err))?;
        signer
            .update(data.as_ref())
            .map_err(|err| SignerError::Update(err))?;

        let signature = signer
            .sign_to_vec()
            .map_err(|err| SignerError::Execute(err))?;

        Ok((
            Self(hash_triplet(previous, signature.as_slice(), data)),
            signature,
            keypair,
        ))
    }

    fn gen_keypair() -> Result<PKey<Private>> {
        let merr = |err| Error::KeyGen(err);
        let keypair = Rsa::generate(Self::RSA_BITS).map_err(merr)?;
        let keypair = PKey::from_rsa(keypair).map_err(merr)?;
        Ok(keypair)
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
