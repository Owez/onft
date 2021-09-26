//! Bespoke protocol and high-level implementation of Non-fungible token (NFT) technology 🚀
//!
//! # Example
//!
//! ```rust
//! use onft::Chain;
//!
//! // create
//! let mut chain = Chain::default();
//! println!("Chain: {:?}", chain);
//!
//! // add block
//! chain.push_data("Hello, world!").unwrap();
//! println!("Chain: {:?}", chain);
//!
//! // verify
//! if let Ok(true) = chain.verify() {
//!     println!("Verified")
//! } else {
//!     eprintln!("Not verified")
//! }
//! ```
//!
//! Check the useful [`examples/`](https://github.com/Owez/onft/tree/master/examples) directory or the item-level documentation for more examples! 😊
//!
//! # Usage
//!
//! Simply add the following to your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! onft = "0.1.0-beta.2"
//! ```
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
/// use onft::prelude::*;
///
/// let block = Block::default();
/// println!("Block: {:?}", block);
/// ```
pub mod prelude {
    pub use crate::error::{SignerError, VerifierError};
    pub use crate::{error, Block, Chain, Hash, Ownership};
}
