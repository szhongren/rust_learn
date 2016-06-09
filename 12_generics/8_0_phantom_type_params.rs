// a phantom type param is one that doesn't show up at runtime, only checked statically at compile time

// Data types can use extra generic type parameters to act as markers or to perform type checking at compile time. These extra parameters hold no storage values, and have no runtime behavior

use std::marker::PhantomData;

// phantom tuple struct that is generic over A, B is phantom
#[derive(PartialEq)]
struct PhantomTuple<A, B> (A, PhantomData<B>);

// A phantom type struct which is generic over `A` with hidden parameter `B`
#[derive(PartialEq)]
struct PhantomStruct<A, B> { first: A, phantom: PhantomData<B> };

// B cannot be used in computations
fn main() {
    let _tuple1: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);
    let _tuple2: PhantomTuple<char, f64> = PhantomTuple('Q', PhantomData);

    // Type specified as `<char, f32>`.
    let _struct1: PhantomStruct<char, f32> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };
    // Type specified as `<char, f64>`.
    let _struct2: PhantomStruct<char, f64> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };
    // type checking will occur so cannot compare the tuples or the structs
}