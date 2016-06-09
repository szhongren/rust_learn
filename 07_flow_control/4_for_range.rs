// for in construct can be used to iterate an Iterator, which is a lazy value generator. this is typically done with range notation a..b

fn main() {
    // `n` will take the values: 1, 2, ..., 100 in each iteration
    // like python range()
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}
