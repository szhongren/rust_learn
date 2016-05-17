// neater way to write the same bounds is with where clause right before the {
use std::fmt::Debug;

trait PrintInOption {
    fn print_in_option(self);
}

// in this case, the bound cannot be expressed without a where clause
impl<T> PrintInOption for T where
Option<T>: Debug {
    fn print_in_option(self) {
        println!("{:?}", Some(self));
    }
}

fn main() {
    let vec = vec![1, 2, 3];

    vec.print_in_option();
}