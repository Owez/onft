//! Contains [Hash](struct@Hash) and implementations

use crate::error::{Error, SignerError, VerifierError};
use crate::{Block, Result, DEFAULT_GENESIS};
use openssl::pkey::{HasPublic, PKey, PKeyRef, Private};
use openssl::{sha::Sha256, sign::Signer, sign::Verifier};

/// Hash for a block allowing full blockchain usage
///
/// # Using
///
/// Due to simplicity and the inherent low-level nature of dealing with hashes
/// directly, it's recommended that you instead use the [Block](crate::Block)
/// interface or even higher to the typical [Chain](crate::Chain) if at all
/// possible. You still can however, in high level terms, do the following
/// directly to a this data item:
///
/// - Create a genesis hash: [Hash::default]
/// - Create a hash containing hashed data: [Hash::new]
/// - Verify a hash: [Hash::verify]
/// - Get the length of a hash signature: [Hash::SIG_LEN]
///
/// # Example
///
/// ```rust
/// use onft::prelude::*;
///
/// fn main() -> onft::Result<()> {
///     let genesis_hash = Hash::default();
///
///     let data = BlockData::new("Hello, world!")?;
///     let (new_hash, signature, pkey) = Hash::new(&genesis_hash, &data)?;
///     let verified = new_hash.verify(&genesis_hash, signature, data, &pkey)?;
///
///     if verified {
///         println!("Verified")
///     } else {
///         eprintln!("Not verified")
///     }
///     Ok(())
/// }
/// ```
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
    /// ```rust
    /// use onft::prelude::*;
    ///
    /// fn main() -> onft::Result<()> {
    ///     let genesis_hash = Hash::default();
    ///
    ///     let data = BlockData::new("Hello, world!")?;
    ///     let (new_hash, _, _) = Hash::new(&genesis_hash, data)?;
    ///
    ///     println!("Hash:\n{:?}", new_hash);
    ///     Ok(())
    /// }
    /// ```
    pub fn new(
        previous: impl Into<&'a Hash>,
        data_hash: impl Into<[u8; 32]>,
    ) -> Result<(Self, [u8; Self::SIG_LEN], PKey<Private>)> {
        Self::new_existing_keypair(previous, data_hash, gen_keypair()?)
    }

    /// Verifies current hash using it's known `signature`, the `pkey` public key
    /// and `data` whilst using the `previous` hash.
    ///
    /// # Example
    ///
    /// ```rust
    /// use onft::prelude::*;
    ///
    /// fn main() -> onft::Result<()> {
    ///     let genesis_hash = Hash::default();
    ///
    ///     let data = BlockData::new("Hello, world!")?;
    ///     let (new_hash, signature, pkey) = Hash::new(&genesis_hash, &data)?;
    ///     let verified = new_hash.verify(&genesis_hash, signature, data, &pkey)?;
    ///
    ///     if verified {
    ///         println!("Verified")
    ///     } else {
    ///         eprintln!("Not verified")
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub fn verify(
        &self,
        previous: impl Into<&'a Hash>,
        signature: impl AsRef<[u8]>,
        data_hash: impl Into<[u8; 32]>,
        pkey: &PKeyRef<impl HasPublic>,
    ) -> Result<bool> {
        let mut verifier = Verifier::new_without_digest(pkey).map_err(VerifierError::Create)?;
        let data_hash = data_hash.into();
        let signature_verified = verifier
            .verify_oneshot(signature.as_ref(), &data_hash[..])
            .map_err(VerifierError::Execute)?;

        Ok(if signature_verified {
            self.0 == hash_triplet(previous.into(), signature, data_hash)
        } else {
            false
        })
    }

    /// Creates a new hash from the previous one alongside hte core data included
    /// within the hash, manually inputting the public/private keypair.
    fn new_existing_keypair(
        previous: impl Into<&'a Hash>,
        data_hash: impl Into<[u8; 32]>,
        keypair: PKey<Private>,
    ) -> Result<(Self, [u8; Self::SIG_LEN], PKey<Private>)> {
        let keypair_signer = keypair.clone();

        let mut signer =
            Signer::new_without_digest(&keypair_signer).map_err(SignerError::Create)?;

        let data_hash = data_hash.into();
        let mut signature = [0; Self::SIG_LEN];
        signer
            .sign_oneshot(&mut signature, &data_hash[..])
            .map_err(SignerError::Update)?;

        Ok((
            Self(hash_triplet(previous.into(), signature, data_hash)),
            signature,
            keypair,
        ))
    }
}

impl Default for Hash {
    /// Creates default genesis hash.
    fn default() -> Self {
        Self(DEFAULT_GENESIS)
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

impl From<[u8; 32]> for Hash {
    fn from(inner: [u8; 32]) -> Self {
        Self(inner)
    }
}

fn gen_keypair() -> Result<PKey<Private>> {
    PKey::generate_ed25519().map_err(Error::KeyGen)
}

fn hash_triplet(previous: &Hash, signature: impl AsRef<[u8]>, data_hash: [u8; 32]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(&previous.0[..]);
    hasher.update(signature.as_ref());
    hasher.update(&data_hash[..]);
    hasher.finish()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::BlockData;

    #[test]
    fn gen_keypair() {
        super::gen_keypair().unwrap();
    }

    #[test]
    fn create_verify_hash() {
        let data = BlockData::new("Hello, world!").unwrap();
        let (hash, signature, pkey) = Hash::new(&Hash::default(), &data).unwrap();
        let verified = hash
            .verify(&Hash::default(), signature, data, &pkey)
            .unwrap();

        if !verified {
            panic!("Valid hash not verified successfully")
        }
    }
}
