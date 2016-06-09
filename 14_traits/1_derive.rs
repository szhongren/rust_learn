// compiler can provide basic implementations for some traits with the #[derive] attribute
// can be manually implemented if more complex behavior is required
// derivable traits:
// - comparable traits: Eq, PartialEq, Ord, PartialOrd
// - Clone, to create T from &T via a copy
// - Hash, to compute a hash from &T
// - Default, to create empty instance of a type
// - Zero, to create zero instance of a numeric type
// - Debug, to format a value with {:?}
// Centimeters, a tuple struct that can be compared
#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64);

// inches, tuple struct that can be printed
#[derive(Debug)]
struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let &Inches(inches) = self;

        Centimeters(inches as f64 * 2.54)
    }
}

// seconds, a tuple struct no additional attributes
struct Seconds(i32);

fn main() {
    let _one_second = Seconds(1);

    // seconds can't be printed without Debug trait
    // println!("One second looks like: {:?}", _one_second);

    // seconds can't be compared without implementing PartialEq
    // let _this_is_true = (_one_second == _one_second);

    let foot = Inches(12);

    println!("One foot equals {:?}", foot);

    let meter = Centimeters(100.0);

    let cmp =
    if foot.to_centimeters() < meter {
        "smaller"
    } else {
        "bigger"
    };

    println!("one foot is {} than one meter.", cmp);
}