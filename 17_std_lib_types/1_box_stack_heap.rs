// all values in Rust are stack allocated by default.
// can be placed in the heap by boxing, which is a smart pointer to a value of type T
// when a box goes out of scope, the inner object is destroyed, and the memory in the heap is freed
// boxed values can be dereferenced with *
use std::mem;

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn origin() -> Point {
    Point{ x: 0.0, y: 0.0 }
}

fn boxed_origin() -> Box<Point> {
    // allocate this Point in the heap
    Box::new(Point{ x: 0.0, y: 0.0 })
}

fn main() {
    // all the type annotations are superfluous
    // stack allocated vars
    let point: Point = origin();
    let rectangle: Rectangle = Rectangle {
        p1: origin(),
        p2: Point {x: 3.0, y: 4.0 }
    };

    // heap allocated Rectangle
    let boxed_rectangle: Box<Rectangle> = Box::new(Rectangle {
        p1: origin(),
        p2: origin()
    });

    // the output of functions can be boxed
    let boxed_point: Box<Point> = Box::new(origin());

    // double indirection
    let box_in_a_box: Box<Box<Point>> = Box::new(boxed_origin());

    println!("Point occupies {} bytes in the stack",
             mem::size_of_val(&point));
    println!("Rectangle occupies {} bytes in the stack",
             mem::size_of_val(&rectangle));

    // box size = pointer size
    println!("Boxed point occupies {} bytes in the stack",
             mem::size_of_val(&boxed_point));
    println!("Boxed rectangle occupies {} bytes in the stack",
             mem::size_of_val(&boxed_rectangle));
    println!("Boxed box occupies {} bytes in the stack",
             mem::size_of_val(&box_in_a_box));

    // Copy the data contained in `boxed_point` into `unboxed_point`
    let unboxed_point: Point = *boxed_point;
    println!("Unboxed point occupies {} bytes in the stack",
             mem::size_of_val(&unboxed_point));
}