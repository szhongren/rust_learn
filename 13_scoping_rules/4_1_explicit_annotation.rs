// borrow checker uses explicit lifetime annotations to determine how long refs are valid
// syntax for explicitly annotating a lifetime uses an apostrophe foo<'a>
// using lifetimes requires generics, above means that lifetime of foo does not exceed that of a
// print_refs takes 2 refs to i32 which have different lifetimes a and b, both must be at least as long as the fn print_refs
fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);
}

// a fn which takes no args but has a lifetime param 'a
fn failed_borrow<'a>() {
    let _x = 12;

    // error: _x does not live long enough
    // let y: &'a i32 = &_x;
    // cannot use 'a as an explicit annotation here will fail because &_x has a shorter lifetime than 'a
    // shorter lifetime cannot be coerced into a longer one
}

fn main() {
    // create vars to be borrowed
    let (four, nine) = (4, 9);

    // borrows both vars by taking the refs in the fn
    print_refs(&four, &nine);
    // any input that is borrowed must outlive the borrower
    // 'four and 'nine must > 'print_refs

    failed_borrow();
    // failed_borrow contains no refs to force 'a to be longer than the lifetime of the function, but 'a is longer, so it defaults to static
}