// crate_type attribute can be used to tell the compiler whether a crate is a binary or a library, and crate_name attribute can be used to set the name of the crate

// this crate is a lib
#![crate_type = "lib"]

// crate is called rary
#![crate_name = "rary"]

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
// don't need to pass the --crate-type flag to rustc