// structs have an extra level of visibility with their fields, which defaults to private. can also be overriden with the pub modifier.
mod my {
    // pub struct with pub field of generic type T
    pub struct WhiteBox<T> {
        pub contents: T,
    }

    // pub struct with private field of generic type T
    #[allow(dead_code)]
    pub struct BlackBox<T> {
        contents: T,
    }

    impl<T> BlackBox<T> {
        // pub constructor method
        pub fn new(contents: T) -> BlackBox<T> {
            BlackBox {
                contents: contents,
            }
        }
    }
}

fn main() {
    // pub structs with pub fields can be constructed as usual
    let white_box = my::WhiteBox {contents: "public info" };

    // and their fields can be normally accessed
    println!("The white box contains: {}", white_box.contents);

    // structs with private fields can be constructed with public constructors
    let _black_box = my::BlackBox::new("classified information");
}
