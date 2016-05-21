// previous examples have been convenient, but sometimes Options and Results have to interact
// first attempt conveniently uses unwrap with the bad errors it results in
fn double_first(vec: Vec<&str>) -> i32 {
    // whan if vec is empty
    let first = vec.first().unwrap();

    // what if element does not parse to a number
    2 * first.parse::<i32>().unwrap()
}

fn main() {
    let numbers = vec!["93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("The first doubled is {}", double_first(numbers));
    println!("The first doubled is {}", double_first(empty));
    println!("THe first doubled is {}", double_first(strings));
}