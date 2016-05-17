// numeric literals can be type annotated by adding the type as a suffix, with the exceptions of usize and isize
// type of unsuffixed numeric literals will depend on how they are used, otherwise i32 or f64 by default
use std::mem::size_of_val;

fn main() {
    // suffixed literals, type known at initialization
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // unsuffixed literal, type depends on use
    let i = 1;
    let f = 1.0;

    // size_of_val() returns the size of a variable in bytes
    // vars are passed by ref to size_of_val
    println!("size of x in bytes: {}", size_of_val(&x));
    println!("size of y in bytes: {}", size_of_val(&y));
    println!("size of z in bytes: {}", size_of_val(&z));
    println!("size of i in bytes: {}", size_of_val(&i));
    println!("size of f in bytes: {}", size_of_val(&f));
    
}
