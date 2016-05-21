// for pointers, a distinction between destructuring and dereferencing needs to be made because they are different concepts, also as compared to a language like C
// dereferencing uses *
// destructuring uses &, ref, and ref mut
fn main() {
    // let ref reference = 4; is equiv to the below
    // reference is a smart pointer to the value 4
    let reference = &4;

    match reference {
        // if reference is pattern matched against &val, it results in a comparison like &i32 : &val
        // dropping the & will assign the i32 to val
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    // to avoid the &, we need to dereference before matching, which is preferred
    // better to match on a concrete value instead of a reference
    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    // what if you don't start with a reference? reference was a ref because the right side was already a reference (&4). the below is not a reference
    let _not_a_reference = 3;

    // rust provides ref keyword for this purpose. it modifies the assignment so that a reference is created for the element, and this reference is assigned
    let ref _is_a_reference = 3;

    // accordingly, by defining 2 values without references, references can be retrieved via ref and ref mut
    let value = 5;
    let mut mut_value = 6;

    // When matching a value of type T, an identifier pattern i will, on a successful match, move the value out of the original input and into i. Thus we can always conclude in such a case that i has type T (or more succinctly, “i: T”).

    // For some types T, known as copyable T (also pronounced “T implements Copy”), the value will in fact be copied into i for such identifier patterns. (Note that in general, an arbitrary type T is not copyable.)

    // Either way, such pattern bindings do mean that the variable i has ownership of a value of type T.

    // the i32 type implements Copy, so here the value is copied into r

    // When matching an L-value of type T, a ref-pattern ref i will, on a successful match, merely borrow a reference into the matched data. In other words, a successful ref i match of a value of type T will imply that i has the type of a reference to T (or more succinctly, “i: &T”).

    // use ref keyword to create a reference that borrows the var binding, since sometimes you cannot take ownership of the original variable. the ref keyword in this case is not required
    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    // use ref mut similarly
    match mut_value {
        ref mut m => {
            // got a reference, which we have to dereference before we can change the value
            *m += 10;
            println!("We added 10. mut_value: {:?}", m);
        },
    }
}
