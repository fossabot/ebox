extern crate cmake;

use cmake::Config;

fn main() {
    let dst = Config::new("solidity").build();

    println!("cargo:rust-link-search=native={}/lib", dst.display());
}
