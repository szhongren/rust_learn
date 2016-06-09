// useful method of unit conversions can be examined byimplementing Add with a phantom type param
use std::ops::Add;
use std::marker::PhantomData;

// create void enums to define unit types
#[derive(Debug, Clone, Copy)]
enum Inch {}

#[derive(Debug, Clone, Copy)]
enum Mm {}

/// Length is a type with phantom type param Unit
#[derive(Debug, Clone, Copy)]
struct Length<Unit> (f64, PhantomData<Unit>);

/// Add trait defines the behavior of the + operator
impl<Unit> Add for Length<Unit> {
    type Output = Length<Unit>;

    fn add(self, rhs: Length<Unit>) -> Length<Unit> {
        Length(self.0 + rhs.0, PhantomData)
    }
}


fn main() {
    let one_foot: Length<Inch> = Length(12.0, PhantomData);
    let one_meter: Length<Mm> = Length(1000.0, PhantomData);

    // `+` calls the `add()` method we implemented for `Length<Unit>`.
    //
    // Since `Length` implements `Copy`, `add()` does not consume
    // `one_foot` and `one_meter` but copies them into `self` and `rhs`.
    let two_feet = one_foot + one_foot;
    let two_meters = one_meter + one_meter;

    // Addition works.
    println!("one foot + one_foot = {:?}", two_feet);
    println!("one meter + one_meter = {:?}", two_meters);
}