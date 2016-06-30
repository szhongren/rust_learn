// Process struct represents a running child process and exposes the stdin, stdout, and stderr handles
// these are used to interact with the underlying process with pipes
use std::error::Error;
use std::io::prelude::*;
use std::process::{Command, Stdio};

static PANGRAM: &'static str =
"the quick brown fox jumped over the lazy dog\n";

fn main() {
    // spawn the wc command
    let process = match
    Command::new("wc")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn() {
            Err(why) => panic!("couldn't spawn wc: {}", why.description()),
            Ok(process) => process,
        };

    // write a string to the stdin of wc
    // stdin has type Option<ChildStdin> but since we know this instance must have one,
    // we can directly unwrap it
    match process.stdin.unwrap().write_all(PANGRAM.as_bytes()) {
        Err(why) => panic!("couldn't write to wc stdin: {}", why.description()),
        Ok(_) => println!("sent program to wc"),
    }

    // because stdin does not live after the above calls, it is dropped and the pipe is closed
    // this is very important, otherwise wc wouldn't start processing input we just sent
    // stdout field also has type Option<ChildStdout> so we need to unwrap
    let mut s = String::new();
    match process.stdout.unwrap().read_to_string(&mut s) {
        Err(why) => panic!("couldn't read wc stdout: {}", why.description()),
        Ok(_) => print!("wc responded with:\n{}", s),
    }
}