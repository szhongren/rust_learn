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
	
}
