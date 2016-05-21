// while keyword can be used to loop until a condition is met
fn main() {
    // counter var
    let mut n = 1;

    // loop while n < 101
    'fizzbuzz: while n < 101 {
        if n % 15 == 0 {
            println!("FizzBuzz");
        } else if n % 3 == 0 {
            println!("Fizz");
        } else if n % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", n);
        }

        n += 1;
    }
}
