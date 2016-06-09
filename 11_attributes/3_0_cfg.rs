// conditional compilation is possible through 2 different operators:
// cfg attribute: #[cfg(...)] in attribute position
// cfg! macro: cfg!(...) in boolean expressions

// this fn only gets compiled if target OS is linux
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are running linux!");
}

// this fn only gets compiled if target not linux
#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("You are not running linux!");
}

fn main() {
    are_you_on_linux();

    println!("Are you sure?");
    if cfg!(target_os = "linux") {
        println!("Yes, it's definitely linux!");
    } else {
        println!("Yes, it's definitely not linux!");
    }
}
