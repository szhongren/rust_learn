// macros can use + in the arg list to indicate that an arg may repeat at least once, or * to indicate that the arg may repeat 0 or more times
// here, surrounding the matcher with $(..),+ will match one or more expressions, separated by commas
// calcs the min of any number of args
macro_rules! find_min {
    ($x: expr) => ($x);
    ($x: expr, $($y: expr), +) => (
        std::cmp::min($x, find_min!($($y), +))
    ) // ; optional since this is the last case
}

fn main() {
    println!("{}", find_min!(1u32));
    println!("{}", find_min!(1u32 + 2 , 2u32));
    println!("{}", find_min!(5u32, 2u32 * 3, 4u32));
}
