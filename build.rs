extern crate cmake;

use cmake::Config;
use std::env;

fn main() {
    let dst = Config::new("solidity")
        .define("TESTS", "OFF")
        .define("TOOLS", "OFF")
        .define("USE_Z3", "OFF")
        .define("USE_CVC4", "OFF")
        .build();

    for lib in vec!["solc", "solidity", "yul", "langutil", "evmasm", "devcore"] {
        println!(
            "cargo:rustc-link-search=native={}/build/lib{}",
            dst.display(),
            lib
        );
        println!("cargo:rustc-link-lib=static={}", lib);
    }

    println!("cargo:rustc-link-search=native={}/lib", dst.display());

    println!(
        "cargo:rustc-link-search=native={}/build/deps/lib",
        dst.display()
    );
    println!("cargo:rustc-link-lib=static=jsoncpp");

    println!("cargo:rustc-link-search=/usr/lib/");
    println!("cargo:rustc-link-lib=boost_system");
    println!("cargo:rustc-link-lib=boost_filesystem");
    println!("cargo:rustc-link-lib=boost_regex");

    if let Some(cpp_stdlib) = get_cpp_stdlib() {
        println!("cargo:rustc-link-lib={}", cpp_stdlib);
    }
}

fn get_cpp_stdlib() -> Option<String> {
    env::var("TARGET").ok().and_then(|target| {
        if target.contains("msvc") {
            None
        } else if target.contains("darwin") {
            Some("c++".to_string())
        } else if target.contains("freebsd") {
            Some("c++".to_string())
        } else if target.contains("musl") {
            Some("static=stdc++".to_string())
        } else {
            Some("stdc++".to_string())
        }
    })
}
