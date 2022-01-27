extern crate cmake;

use cmake::Config;

fn main() {
    let dst = Config::new("solidity")
        .define("TESTS", "OFF")
        .define("TOOLS", "OFF")
        .define("USE_Z3", "OFF")
        .define("USE_CVC4", "OFF")
        .build();

    println!("cargo:rust-link-search=native={}/lib", dst.display());
}
