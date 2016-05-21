// format!: writes formatted text to string
// print!: same as format! but prints to the console
// println!: same as print! but adds a \n at the end

fn main() {
    // in general, {} in print strings will be replaced with args that are stringified
    println!("{} days", 31);

    // without a suffix, 31 above becomes an i32 by default. you can spec the type with a suffix

    // an optional pattern is positional arguments
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // another pattern is named arguments
    println!("{subject} {verb} {object}",
                object="the lazy dog",
                subject="the quick brown fox",
                verb="jumps over");

	// special formatting can be specified after a ':'
	println!("{} of {:b} people know binary, the other half don't.", 1, 2);

	// you can right-align text with a specified width. this will output 5 whitespaces and a 1, right aligned (>) while width$ means that it is a var
	println!("{number:>width$}", number=1, width=6); // print number, right aligned(>), width of 6
	// you can also pad numbers with extra 0's, this will output 000001
	println!("{number:>0width$}", number=1, width=6);

	// it will even check for correct number of args
	println!("My name is {0}, {1} {0}.", "Bond", "James");

	// create a struct with contains an i32 called structure
	struct Structure(i32);

	// you need special handling to print user defined types, so the below won't work
	// println!("This is a struct '{}'", Structure(32));

	// std::fmt contains many traits that govern the displaying of text
	// fmt::Debug uses the {:?} marker, for debugging
	// fmd::Display uses the {} marker, for user-friendly things

}
