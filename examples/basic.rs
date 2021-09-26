//! Simple example showing creation, addition and verification on a Onft blockchain

use onft::Chain;

fn main() {
    // create
    let mut chain = Chain::default();
    println!("Initial chain:\n{:?}", chain);

    // add block
    chain.push_data("Hello, world!").unwrap();
    println!("New chain:\n{:?}", chain);

    // verify
    if chain.verify().unwrap() {
        println!("Verified chain")
    } else {
        eprintln!("Chain failed verification!")
    }
}