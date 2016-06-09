// associated items refers  to a set of rulse pertaining to items of various types
// extension to trait generics, allows traits to internally define new items
struct Container(i32, i32);

// a trait which checks if 2 items are stored inside of container
// also retrieves first or last value
trait Contains<A, B> {
    fn contains(&self, &A, &B) -> bool; // explicitly requires A or B
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains<i32, i32> for Container {
    // true if numbers stored are equal
    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }

    // grab the first number
    fn first(&self) -> i32 {
        self.0
    }

    fn last(&self) -> i32 {
        self.1
    }
}

// C contains A and B, so having to express A and B again is a nuisance
fn difference<A, B, C>(container: &C) -> i32 where
C: Contains<A, B> {
    container.last() - container.first()
}

fn main() {
    let number_1 = 3;
    let number_2 = 10;

    let container = Container(number_1, number_2);

    println!("Does container contain {} and {}: {}", &number_1, &number_2, container.contains(&number_1, &number_2));
    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());

    println!("The difference is: {}", difference(&container));
}