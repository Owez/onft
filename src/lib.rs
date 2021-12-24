//! Bespoke toolkit for Non-fungible token (NFT) technology 🚀
//!
//! # What is Onft?
//!
//! Instead of forcing a consensus algorithm or peer networking on you, Onft provides you with the tools to create a reliable and *fast* NFT system 👐
//!
//! This allows you to focus on implementing the important stuff, as well as getting benefits such as automatic improvements and updates over this project's lifecycle, whilst still being fully standardized.
//!
//! # Example
//!
//! Simple creating, adding and verifying procedure based upon the typical [Chain] flow:
//!
//! ```rust
//! use onft::prelude::*;
//!
//! // create
//! let mut chain = Chain::default();
//! println!("Chain: {:?}", chain);
//!
//! // add block
//! chain.push("Hello, world!").unwrap();
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
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/Owez/onft/master/logo.png",
    html_favicon_url = "https://raw.githubusercontent.com/Owez/onft/master/logo.png"
)]

pub mod error;

mod block;
mod chain;
mod hash;

pub use block::{Block, BlockData, Ownership};
pub use chain::Chain;
pub use error::Result;
pub use hash::Hash;

/// Defines the breaking ABI protocol version this release uses for (de)serialization
#[cfg(feature = "serde")]
pub const PROTO_VERSION: u8 = 1;

/// Defines the default initializer for SHA-256 hashes, used for genesis hashes
pub(crate) const DEFAULT_GENESIS: [u8; 32] = [
    66, 108, 111, 111, 100, 121, 32, 103, 101, 110, 101, 115, 105, 115, 32, 98, 108, 111, 99, 107,
    32, 109, 101, 115, 115, 97, 103, 101, 115, 46, 46, 46,
];

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
    pub use crate::{error, Block, BlockData, Chain, Hash, Ownership};
}
