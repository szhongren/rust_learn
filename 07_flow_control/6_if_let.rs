// in some cases match usage can be awkward, like below
// Make `optional` of type `Option<i32>`

// let optional = Some(7);
//
// match optional {
//     Some(i) => {
//         println!("This is a really long string and `{:?}`", i);
//         // ^ Needed 2 indentations just so we could destructure
//         // `i` from the option.
//     },
//     _ => {},
//     // ^ Required because `match` is exhaustive. Doesn't it seem
//     // like wasted space?
// };

// if let is cleaner for this case and allows various failure options to be specified
fn main() {
    // all have type Option<i32> (could be Some(v) or None)
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    // if let means: if 'let' destructures number into 'Some(i)', eval the block of code
    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }

    // if you need to specify a failure, use an else
    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        println!("Didn't match a number, let's go with a letter!");
    }

    // provide an altered failing condition
    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("Matched {:?}!", i);
    // destructure failed, eval the else if to see if the alternate failure branch should be taken
    } else if i_like_letters {
        println!("Didn't match a number, let's go with a letter!");
    } else {
        // condition was false, this branch is the default
        println!("I don't like letters, let's go with an emoticon.");
    }
}
