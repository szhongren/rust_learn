// arrays are collections in contiguous memory, created using brackets
// size is known at compile time and part of the type signature
// slices are similar to arrays but size is not known at compile time, first word is a ptr to data and second word is length of the slice
use std::mem;

// this fn borrows a slice, which is a section of an array, signature of &[T]
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len()); // no return value here
}

fn main() {
    // fixed size array
    let xs: [i32; 5] = [1, 2, 3, 4 , 5];

    // all elems can be init to same value
    let ys: [i32; 500] = [0; 500];

    // indexing starts at 0
    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    // len() returns the size of the array
    println!("size of the array: {}", xs.len());

    // arrays are stack allocated
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    // arrays can be automatically borrowed as slices
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    // slices can point to a section of an array
    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1 .. 4]);

    // out of bound indexing yields a panic
    println!("{}", xs[5]);
}
