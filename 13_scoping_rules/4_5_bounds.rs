// since all generic types can be bounded, lifetimes themselves can be bounded too
// T: 'a means all refs in T must outlive 'a
// T: Trait + 'a means T must impl Trait and all refs in T must outlive 'a
use std::fmt::Debug; // trait to bound with

#[derive(Debug)]
struct Ref<'a, T: 'a>(&'a T);
// Ref contains a ref to a generic type T that has an unknown lifetime 'a
// T is bounded such that any refs in T must outlive 'a, and the lifetime of Ref cannot exceed 'a

// a generic function which prints using the Debug trait
fn print<T>(t: T) where
T: Debug {
    println!("print: t is {:?}", t);
}

// here a ref to T is taken where T implements Debug and all refs in T outlive 'a, which in turn outlives the function
fn print_ref<'a, T>(t: &'a T) where
T: Debug + 'a {
    println!("print_ref: t is {:?}", t);
}

fn main() {
    let x = 7;
    let ref_x = Ref(&x);

    print_ref(&ref_x);
    print(ref_x);
}