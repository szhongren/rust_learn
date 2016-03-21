// A crate is a compilation unit in Rust. Whenever rustc some_file.rs is called, some_file.rs is treated as the crate file. If some_file.rs has mod declarations in it, then the contents of the module files will get merged with the crate file before running the compiler over it. In other words, modules do not get compiled individually, only crates get compiled.

// A crate can be compiled into a binary or into a library. By default, rustc will produce a binary from a crate. This behavior can be overridden by passing the --crate-type flag to rustc with arg lib

// rary.rs
pub fn public_function() {
    println!("called rary's public_function()");
}

fn private_function() {
    println!("called rary's private_function()");
}

pub fn indirect_access() {
    print!("called rary's indirect_access(), that\n> ");
    private_function();
}

// libs get prefixed with lib, and by default they get named after their crate file. this default name can be overriden with the crate_name attribute
// this file will produce library.rlib
