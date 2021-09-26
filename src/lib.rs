//! Bespoke protocol and high-level implementation of Non-fungible token (NFT) technology 🚀
//!
//! # Licensing
//!
//! This project is dual-licensed under both the [MIT](https://en.wikipedia.org/wiki/MIT_License) and [Apache](https://en.wikipedia.org/wiki/Apache_License) licenses, so feel free to use either at your discretion.

#![deny(unsafe_code)]
#![warn(missing_docs)]

pub mod error;

mod block;
mod chain;
mod hash;

pub use block::{Block, Ownership};
pub use chain::Chain;
pub use error::Result;
pub use hash::Hash;

/// Light prelude layer which provides direct imports
///
/// # When to use
///
/// This prelude module is here for simplicity in some developer's workflows,
/// but generally it's best to just import data items from this library directly
/// due to the low amount contained within. In the end, it's up to developer
/// tastes. ❤️
///
/// # Example
///
/// This gives access to in times simpler usage of this crate, for example:
///
/// ```rust
/// use onft::prelude::*
///
/// fn main() {
///     let block = Block::default().unwrap();
///     println!("Block:\n{:?}", block);
/// }
/// ```
pub mod prelude {
    pub use crate::error::{SignerError, VerifierError};
    pub use crate::{error, Block, Chain, Hash, Ownership};
}
