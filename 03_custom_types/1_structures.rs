// rust custom data types through keywords: struct, enum
// constants can also be created with the const and static keywords

// 3 types of structs that can be created with struct keyword
// 1. tuple struct, which is basically a named tuple
// 2. classic C struct
// 3. unit structs, which are fleld-less, are useful for generics

// a unit struct
struct Nil;

// a tuple struct
struct Pair(i32, f64);

// struct with 2 fields
struct Point {
    x: f64,
    y: f64,
}

// structs can be fields of another struct
// allow dead_code allows the following item to never be used
#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn main() {
    // inst a point
    let point: Point = Point { x: 0.3, y: 0.4 };

    // access the fields of the point
    println!("point coords: ({}, {})", point.x, point.y);

    // destructure the point using let
    let Point { x: my_x, y: my_y } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        p1: Point { x: my_x, y: my_y },
        p2: point,
    };

    // inst a unit struct
    let _nil = Nil;

    // inst a tuple struct
    let pair = Pair(1, 0.1);

    // destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}
