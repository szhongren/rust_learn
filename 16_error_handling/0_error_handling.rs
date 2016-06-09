// error handling is what it sounds like
// simplest way is just a panic!, which prints error message, starts unwinding the task, and exits the program
fn give_princess(gift: &str) {
    if gift == "snake" {
        panic!("AAHHHHHH");
    }

    println!("I love {}s!", gift);
}

fn main() {
    give_princess("teddy bear");
    give_princess("snake");
}