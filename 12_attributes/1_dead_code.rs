// attributes are metadata applied to some module, crate, or item. can be used for:
// - conditional compilation of code
// - set crate name, version, or type
// - disable linter warnings
// - enable compiler features (macros, glob imports, etc)
// - link to foreign libs
// - mark functions as unit tests
// - mark functions that will be part of a benchmark

// attributes that apply to whole crate have #![crate_attribute] while module or item only attributes have #[crate_attribute]

// different syntaxes:
// 1. #[attribute = "value"]
// 2. #[attribute(key = "value")]
// 3. #[attribute(value)]

// compiler provides a dead_code lint that warns about unused functions, which can be disabled
fn used_function() {}

#[allow(dead_code)]
fn unused_function() {}

fn noisy_unused_function() {}

fn main() {
    used_function();
}
