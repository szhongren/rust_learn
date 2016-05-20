// when working with generics, the type params must often use traits use bounds to stipulate what functionality a type implements
// this restricts the generic to types that confrom to the bounds, aka types that have a certain trait
use std::fmt::Debug;

trait hasArea {
    fn area(&self) -> f64;
}

impl hasArea for Rectangle {
    fn area(&self) -> f64 { self.length * self.height }
}

#[derive(Debug)]
struct Rectangle {
    length: f64, height: f64
}
#[allow(dead_code)]
struct Triangle {
    length: f64, height: f64
}

// generic T here must impl Debug
fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

// T must impl hasArea, and any fn that meets the bound can access hasArea's fn area'
fn area<T: hasArea>(t: &T) -> f64 {
    t.area()
}

fn main() {
    let rect = Rectangle { length: 3.0, height: 4.0 };
    let _tri = Triangle { length: 3.0, height: 4.0 };

    print_debug(&rect);
    println!("Area: {}" , area(&rect));
}