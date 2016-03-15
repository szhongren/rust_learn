// type statement can be used to give a new name to an existing type, which must have CamelCase names, or compiler will raise a warning
// NanoSecond is a new name for u64
type NanoSecond = u64;
type Inch = u64;

// attrib to silence warnings
#[allow(non_camel_case_types)]
type u64_t = u64;

// as above
#[allow(trivial_numeric_casts)]
fn main() {
    // NanoSecond = Inch = u64_t = u64
    let nanoseconds: NanoSecond = 5 as u64_t;
    let inches: Inch = 2 as u64_t;

    // aliases don't provide extra type safety because aliases are not new types
    println!("{} nanoseconds + {} inches = {} units?", nanoseconds, inches, nanoseconds + inches);
    // main use is to reduce typing and easier programming
}
