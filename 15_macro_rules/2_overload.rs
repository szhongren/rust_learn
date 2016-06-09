// macros can be overloaded to accept different combinations of args, which can make it work like a match block
// will compare $left and $right differently depending on how it is invoked
macro_rules! test {
    // args don't need separating commas, and any template can be used, non identifier will be matched literally
    ($left: expr; and $right: expr) => (
        println!("{:?} and {:?} is {:?}",
                    stringify!($left),
                    stringify!($right),
                    $left && $right)
    );
    // each arm must end with a ;
    ($left: expr; or $right: expr) => (
        println!("{:?} or {:?} is {:?}",
                    stringify!($left),
                    stringify!($right),
                    $left || $right)
    )
}

fn main() {
    test!(1i32 + 1 == 2i32; and 2i32 * 2 == 4i32);
    test!(true; or false);
}
