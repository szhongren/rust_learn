// rust passes ownership by default, but if you pass by reference (&) you only borrow the value instead of taking ownership
// this fn takes ownership of a box and destroys it
fn eat_box(boxed_int: Box<i32>) {
    println!("Destroying box that contains {}", boxed_int);
}

// this fn borrows an i32
fn borrow_box(borrowed_int: &i32) {
    println!("This int is {}", borrowed_int);
}

fn main() {
    // create a boxed int
    let boxed_int = Box::new(5);

    // borrow the contents of the box
    borrow_box(&boxed_int);
    borrow_box(&boxed_int);

    {
        // take a ref to the data in the box
        let _ref_to_int: &i32 = &boxed_int;

        // below code gives compile error since boxed_int has been borrowed
        // eat_box(boxed_int);
        // _ref_to_int goes out of scope and is no longer borrowed
    }
    eat_box(boxed_int);
}