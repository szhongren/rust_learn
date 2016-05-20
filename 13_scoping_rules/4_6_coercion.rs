// a longer lifetime can be coerced into a shorter one so that it works in a scope it normally doesn't work in
// here Rust infers a lifetime that is as short as possible
// 2 refs are coerced to that lifetime
fn multiply<'a>(first: &'a i32, second: &'a i32) -> i32 {
    first * second
}

// <a: 'b, 'b> means 'a is at leats as long as 'b
// here, we take in an &'a i32 and return a &b i32
fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 {
    first
}

fn main() {
    let first = 2;
    {
        let second = 3;
        println!("the product is {}", multiply(&first, &second));
        println!("{} is the first", choose_first(&first, &second));
    };
}