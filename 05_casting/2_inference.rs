// type inference engine is pretty smart and does more than look at the rval type during an init
fn main() {
    // because of suffix, compiler knows that elem is type u8
    let elem = 5u8;

    // create an empty vector
    let mut vec = Vec::new();
    // at this point the compiler does not know the type of vec, just that it's a vec of something Vec<_>

    // insert elem in vector
    vec.push(elem);
    // now the compiler knows that vec is a Vec<u8>

    println!("{:?}", vec);
}
