# Onft

Bespoke protocol and high-level implementation of Non-fungible token (NFT) technology 🚀

## Usage

```rust
use onft::Chain;

// create
let mut chain = Chain::default();
println!("Chain: {:?}", chain);

// add block
chain.push_data("Hello, world!").unwrap();
println!("Chain: {:?}", chain);

// verify
if let Ok(true) = chain.verify() {
    println!("Verified")
} else {
    eprintln!("Not verified")
}
```

## Licensing

This project is dual-licensed under both the [MIT](https://en.wikipedia.org/wiki/MIT_License) and [Apache](https://en.wikipedia.org/wiki/Apache_License) licenses, so feel free to use either at your discretion.
