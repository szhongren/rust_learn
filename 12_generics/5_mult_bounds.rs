// multiple bounds applide with a +, and different types separated with ,
use std::fmt::{Debug, Display};

fn cmp_prints<T: Debug + Display>(t: &T) {
    println!("Debug: `{:?}`", t);
    println!("Display: `{}`", t);
}

fn cmp_types<T: Debug, U: Debug>(t: &T, u: &U) {
    println!("t: `{:?}`", t);
    println!("u: `{:?}`", u);
}

fn main() {
    let string = "words";
    let array = [1, 2, 3];
    let vec = vec![1, 2, 3];

    cmp_prints(&string);

    cmp_types(&array, &vec);
}