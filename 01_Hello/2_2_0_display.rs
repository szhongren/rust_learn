// note that fmt::Display is not implemented for any generic containers

use std::fmt; // importing std::fmt for use

// derive debug so it can be contrasted with display
#[derive(Debug)]
struct MinMax(i64, i64);

// implement display for MinMax
impl fmt::Display for MinMax {
    // this trait requires fmt() to have this exact signature
    // f is as mutable fmt::Formatter, or the output stream
    // returns a fmt::Result that indicates success or failure
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // use self.num to refer to each positional data point
        write!(f, "({}, {})", self.0, self.1) // no ; means that this statement is the return value
    }
}

// define a struct with named fields
#[derive(Debug)]
struct Point2 {
    x: f64,
    y: f64,
}

// impl fmt::Display for Point2
impl fmt::Display for Point2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y) // ditto as above
    }
}

// you can implement fmt::Binary too as another trait

fn main() {
    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and the small is {small}.", small = small_range, big = big_range);

    let point = Point2{ x: 3.3, y: 7.2};

    println!("Compare structures:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

}
