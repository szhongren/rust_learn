// when dealing with resources default behavior is to transfer then during assignments or function calls, but sometimes we need to copy them as well
// Clone trait helps us to do this: use the .clone() method defined by the Clone trait
// unit struct without resources
#[derive(Debug, Clone, Copy)]
struct Nil;

// a tuple struct with resources that implements the Clone trait
#[derive(Clone, Debug)]
struct Pair(Box<i32>, Box<i32>);

fn main() {
    // instantiate Nil
    let nil = Nil;
    // copy Nil, no resources to move
    let copied_nil = nil;

    // both nils can be used independently
    println!("original: {:?}", nil);
    println!("copy: {:?}", copied_nil);

    // instantiate Pair
    let pair = Pair(Box::new(1), Box::new(2));
    println!("original: {:?}", pair);

    // copy pair into moved_pair, moves resources
    let moved_pair = pair;
    println!("copy: {:?}", moved_pair);

    // error: pair has lost its resources
    // println!("original: {:?}", pair);

    // clone moved_pair into cloned_pair (resources are included)
    let cloned_pair = moved_pair.clone();
    // drop the original pair with Drop
    drop(moved_pair);

    // result from cloned_pair can still be used
    println!("clone: {:?}", cloned_pair);
}