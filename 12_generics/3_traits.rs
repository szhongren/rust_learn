// traits can also be generic
// non-copyable types
struct Empty;
struct Null;

// a trait generic over T
trait DoubleDrop<T> {
    // def a method on the caller type which takes an additional single param T
    fn double_drop(self, _: T);
}

// impl doubledrop for any generic param T and caller U
// <T, U> part means that we will use 2 types in the generic declaration
impl<T, U> DoubleDrop<T> for U {
    // this method takes ownership of both passed arguments, deallocating both
    fn double_drop(self, _: T) {}
}

fn main() {
    let empty = Empty;
    let null = Null;

    // deallocate empty and null
    empty.double_drop(null);

}