// a trait is a collection of methods for an unknown type Self, and can access other methods declared in the same trait
// traits can be implemented for any data type
struct Sheep { naked: bool, name: &'static str }

trait Animal {
    // static method signature, Self refers to implementor type
    fn new(name: &'static str) -> Self;

    // instance method signatures
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    // traits can provide default method defs
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }

    fn shear(&mut self) {
        if self.is_naked() {
            // implementor methods can use the implementor's trait methods'
            println!("{} is already naked.", self.name());
        } else {
            println!("{} gets a haircut. {}", self.name, self.name());
            self.naked = true;
        }
    }
}

impl Animal for Sheep {
    // Self is the implementor type
    fn new(name: &'static str) -> Sheep {
        Sheep { name: name, naked: false }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "baaaah?"
        } else {
            "beeeeh!"
        }
    }

    // default trait methods can be overriden
    fn talk(&self) {
        // add some quiet contemplation
        println!("{} pauses briefly {}", self.name, self.noise());
    }
}

fn main() {
    // type annotation is necessary in this case
    let mut dolly: Sheep = Animal::new("Dolly");

    dolly.talk();
    dolly.shear();
    dolly.talk();
}