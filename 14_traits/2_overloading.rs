// in rust, operators can be overloaded via traits, aka some ops can be used to accomplish different tasks based on input arguments
// ops are syntactic sugar for method calls
// + calls the add method, part of the Add trait
use std::ops;

struct Foo;
struct Bar;

#[derive(Debug)]
struct FooBar;

#[derive(Debug)]
struct BarFoo;

// here the Add trait is used to specify the functionality of +
// we make Add<Bar> the trait for addition with a RHS of type Bar
// following implements Foo + Bar = FooBar
impl ops::Add<Bar> for Foo {
    type Output = FooBar;

    fn add(self, _rhs: Bar) -> FooBar {
        println!("> Foo.add(Bar) was called");

        FooBar
    }
}

// by reversing the types, we end up implementing non-commutative addition
// here we make Add<Foo> the trait for addition with a RHS of type Foo
impl ops::Add<Foo> for Bar {
    type Output = BarFoo;

    fn add(self, _rhs: Foo) -> BarFoo {
        println!("> Bar.add(Foo) was called");

        BarFoo
    }
}

fn main() {
    println!("Foo + Bar = {:?}", Foo + Bar);
    println!("Bar + Foo = {:?}", Bar + Foo);
}