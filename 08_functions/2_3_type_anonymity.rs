// closures succinctly capture vars from enclosing scopes, which has consequences. for example, using a closure in a function needs generics, because of how they are defined
// When a closure is defined, the compiler implicitly creates a new anonymous structure to store the captured variables inside, meanwhile implementing the functionality via one of the traits: Fn, FnMut, or FnOnce for this unknown type. This type is assigned to the variable which is stored until calling.
// Since this new type is of unknown type, any usage in a function will require generics. However, an unbounded type parameter <T> would still be ambiguous and not be allowed. Thus, bounding by one of the traits: Fn, FnMut, or FnOnce (which it implements) is sufficient to specify its type.
// F must implement Fn for a closure which takes no inputs and returns nothing, which is exactly what is needed for print, and it is generic
fn apply<F>(f: F)
where F: Fn() {
    f()
}

// the above is the same as;
// fn apply<F: Fn()>(f: F) {
//     f()
// }

fn main() {
    let x = 7;

    // capture x into an anonymous type and implement Fn for it, then store that in print
    let print = || println!("{}", x);

    apply(print);
}
