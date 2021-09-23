use crate::{Error, Result, SignerError};
use openssl::pkey::{PKey, Private, Public};
use openssl::{hash::MessageDigest, rsa::Rsa, sha::Sha256, sign::Signer};

pub struct Hash([u8; 32]);

impl Hash {
    pub const RSA_BITS: u32 = 2048;

    pub fn new(previous: impl Into<Hash>, data: impl AsRef<[u8]>) -> Result<(Self, PKey<Private>)> {
        Self::new_existing_keypair(previous, data, Self::gen_keypair()?)
    }

    pub fn verify(
        previous: impl Into<Hash>,
        signature: impl AsRef<[u8]>,
        data: impl AsRef<[u8]>,
        pkey: PKey<Public>,
    ) -> Result<bool> {
        todo!()
    }

    fn new_existing_keypair(
        previous: impl Into<Hash>,
        data: impl AsRef<[u8]>,
        keypair: PKey<Private>,
    ) -> Result<(Self, PKey<Private>)> {
        let keypair_signer = keypair.clone();

        let mut signer =
            Signer::new(msgd(), &keypair_signer).map_err(|err| SignerError::Create(err))?;
        signer
            .update(data.as_ref())
            .map_err(|err| SignerError::Update(err))?;

        let signature = signer.sign_to_vec().map_err(|err| SignerError::Execute(err))?;

        Ok((Self(hash_triplet(previous, signature, data)), keypair))
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

fn hash_triplet(
    previous: impl Into<Hash>,
    signature: impl AsRef<[u8]>,
    data: impl AsRef<[u8]>,
) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(&previous.into().0[..]);
    hasher.update(signature.as_ref());
    hasher.update(data.as_ref());
    hasher.finish()
}
