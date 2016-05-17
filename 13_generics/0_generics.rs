// generics is used to generalize types and functions to multiple data types
// type parameters atre specified with CamelCase, and generics usually T
// concrete type A
struct A;

// first use of A is not preceded by <A>, so Single is also concrete
struct Single(A);

// here <T> precedes first use of T, so SingleGen is a generic type and could be anything
struct SingleGen<T>(T);

fn main() {
    // Single in concrete and explicitly takes A, and unused
    let _s = Single(A);

    // create a var char of type SingleGen<char>
    // contains an 'a'
    let _char: SingleGen<char> = SingleGen('a');

    // type param deduced
    let _t = SingleGen(A);
    let _i32 = SingleGen(6);
    let _char = SingleGen('a');
}