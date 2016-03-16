// methods are functios attached to objects, that have access to the data of the object via the self keyword. defined under an impl block
struct Point {
    x: f64,
    y: f64,
}

// impl block, all Point methods go in here
impl Point {
    // this is a static method, which means they don't need to be called by an instance. generally used as constructors, and simply don't have &self in the signature
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // another static method, taking 2 args
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // this is an instance method, with the &self in the signature
    // &self is syntactic sugor for self: &Self, where Self is the type of the caller object. in this case Self = Rectangle
    fn area(&self) -> f64 {
        // self gives access to the struct fields via the dot operator
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // abs is a f64 method, returns absolute value
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // this method requires the caller object to be mutable, so &mut self desugars to self: &mut Self
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// pair owns resources, 2 heap allocated integers
#[derive(Debug)]
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // this method consumes the resources of the caller object
    // self desugars to self: Self, taking ownership so you can deallocate the heap
    fn destroy(self) {
        // destructure self
        print!("Destroying {:?}", self);

        // first and second go out of scope and get destroyed
    }
}

fn main() {
    let rect = Rectangle {
        // static methods are called using double colons
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // instance methods are called using dot operator
    // &self is always implicitly passed
    println!("rect perimeter: {}", rect.perimeter());
    println!("rect area: {}", rect.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();
}
