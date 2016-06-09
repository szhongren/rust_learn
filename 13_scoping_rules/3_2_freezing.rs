// when data is immutably borrowed, it also freezes, so it can't be modified via the original object till all refs to it go out of scope
fn main() {
    let mut _mutable_integer = 7i32;
    {
        // borrow mutable_integer
        let _large_integer = &_mutable_integer;

        // _mutable_integer is frozen in this scope, cannot use
        // _mutable_integer = 50;

        // _large_integer goes out of scope
    }
    _mutable_integer = 3;
}