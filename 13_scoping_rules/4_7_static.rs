// 'static lifetime is the longest lifetime, lasts for the lifetime of the running program
// can be coerced to a shorter lifetime
// all stored in read only memory of the binary
// make a const with 'static lifetime
static NUM: i32 = 18;

// returns a ref to NUM where its 'static lifetime is coerced to that of the input argument
fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}

fn main() {
    {
        // make a string literal and print it
        let static_string = "I'm in read-only memory.";
        println!("static_string: {}", static_string);

        // when static_string goes out of scope, the ref can no longer be used
        // data remains in the binary
    }
    {
        // make an int to use for coerce_static
        let lifetime_num = 9;

        // coerce NUM to lifetime of lifetime_num
        let coerced_static = coerce_static(&lifetime_num);

        println!("coerced_static: {}", coerced_static);
        println!("lifetime_num: {}", lifetime_num);
    }

    println!("NUM: {} stays accessible!", NUM);
}