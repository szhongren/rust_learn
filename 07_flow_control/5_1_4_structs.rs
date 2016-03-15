// structs can also be destructured, as below
fn main() {
    #[derive(Debug)]
    struct Foo { x: (u32, u32), y: u32 }

    // destructure members of the struct
    let foo = Foo {x: (1, 2), y: 3 };
    let Foo { x: (a, b), y } = foo;

    println!("{:?}", foo);
    println!("a = {}, b = {}, y = {}", a, b, y);

    // you can destructure structs and rename the vars, the order is not important

    let Foo { y: i, x: j } = foo;
    println!("i = {:?}, j = {:?}", i, j);

    // you can also ignore some variables
    let Foo { y, .. } = foo;
    println!("y = {}", y);

    // error because it does not mention the field x
    // let Foo { y } = foo;
}
