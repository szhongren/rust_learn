// iterator trait is used to implement iterators over collections such as arrays
// only requires a method to be defined for the next element, which can be manually defined in an impl block or automatically defined
//for construct turns some collections into iterators with the into_iterator() method
struct Fibonacci {
    curr: u32,
    next: u32,
}

// implement iterator for Fibonacci
impl Iterator for Fibonacci {
    type Item = u32;

    // here we define the sequence with .curr and .next
    // return type is Option<T>
    // when iterator is done, returns None
    // otherwise, next value is wrapped in Some and returned
    // must return Option<T>
    fn next(&mut self) -> Option<u32> {
        println!("> Next called");
        let new_next = self.curr + self.next;

        self.curr = self.next;
        self.next = new_next;

        // no endpoint to Fibonacci, so always returns Some
        Some(self.curr)
    }
}

fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 0, next: 1 }
}

fn main() {
    // 0..3 is an iterator that generates 0, 1, 2
    let mut sequence = 0..3;

    println!("4 consecutive next calls on 0..3");
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());

    // for works through an Iterator until it returns None
    // each Some value is unwrapped and bound to a var: here this is i
    println!("Iterate through 0..3 using for");
    for i in 0..3 {
        println!("> {}", i);
    }

    // the take(n) method reduces an Iterator to its first n terms
    println!("The first 4 terms of the Fib sequence are: ");
    for i in fibonacci().take(4) {
        println!("> {}", i);
    }

    // skip(n) drops the first n terms
    println!("The next 4 terms of the Fib sequence are: ");
    for i in fibonacci().skip(4).take(4) {
        println!("> {}", i);
    }

    let array = [1u32, 3, 3, 7];

    // iter produces an iterator over an array/slice
    println!("Iterate the following array: {:?}", &array);
    for i in array.iter() {
        println!("> {}", i);
    }
}