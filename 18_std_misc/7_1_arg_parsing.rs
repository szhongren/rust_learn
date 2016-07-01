// matching can be used to parse simple args
use std::env;

fn increase(number: i32) {
    println!("{}", number + 1);
}

fn decrease(number: i32) {
    println!("{}", number - 1);
}

fn help() {
    println!("Usage:
match_args <string>
    Check whether given string is the answer.
match_args {{increase|decrease}} <integer>
    Increase or decrease the given integer by one.")
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        // no args passed
        1 => println!("My name is match_args. Try passing some arguments!"),
        // 1 arg passed
        2 => {
            match args[1].parse() {
                Ok(42) => println!("This is the answer!"),
                _ => println!("This is not the answer!"),
            }
        },
        // 1 command and 1 arg passed
        3 => {
            let cmd = &args[1];
            let num = &args[2];
            // parse the number
            let number: i32 = match num.parse() {
                Ok(n) => n,
                Err(_) => {
                    println!("Error: second arg not an integer");
                    help();
                    return;
                },
            };
            // parse the command
            match &cmd[..] {
                "increase" => increase(number),
                "decrease" => decrease(number),
                _ => {
                    println!("Error: invalid command");
                    help();
                },
            }
        },
        // all other cases
        _ => help(),
    }
}