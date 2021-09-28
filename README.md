<p align="center">
    <a href="https://github.com/Owez/onft"><img src="logo.png" width=200></a>
</p>
<p align="center">
    <a href="https://crates.io/crates/onft"><img src="https://img.shields.io/crates/v/onft.svg" alt="Crates.io"></a>
    <a href="https://docs.rs/onft/"><img src="https://docs.rs/onft/badge.svg" alt="Docs"></a>
</p>
<p align="center">
    Bespoke toolkit for Non-fungible token (NFT) technology 🚀
</p>

## What is Onft?

Instead of forcing a consensus algorithm or peer networking on you, Onft provides you with the tools to create a reliable and *fast* NFT system 👐

This allows you to focus on implementing the important stuff, as well as getting benefits such as automatic improvements and updates over this project's lifecycle, whilst still being fully standardized.

## Example

```rust
use onft::Chain;

// create
let mut chain = Chain::default();
println!("Chain: {:?}", chain);

// add block
chain.push("Hello, world!").unwrap();
println!("Chain: {:?}", chain);

// verify
if let Ok(true) = chain.verify() {
    println!("Verified")
} else {
    eprintln!("Not verified")
}
```

Check the useful [`examples/`](https://github.com/Owez/onft/tree/master/examples) directory or the item-level documentation for more examples! 😊

## Usage

Simply add the following to your `Cargo.toml` file:

```toml
[dependencies]
onft = "0.1.0-beta.2"
```

## Licensing

This project is dual-licensed under both the [MIT](https://en.wikipedia.org/wiki/MIT_License) and [Apache](https://en.wikipedia.org/wiki/Apache_License) licenses, so feel free to use either at your discretion.
