// 2 types of strings in Rust, String and &str
// String is stored as a vector of bytes, but guaranteed to always be a valid
// UTF-8 sequence. Heap allocated, growable and not null-terminated
// &str is a slice that always points to a valid UTF-8 seq, can be used to view
// into a String
fn main() {
    // ref to string allocated in read only memory
    let pangram: &'static str = "the quick brown fox jumps over the lazy dog";
    println!("Pangram: {}", pangram);

    // iter over words in reverse, no new string is allocated
    println!("Words in reverse");
    for word in pangram.split_whitespace().rev() {
        println!("> {}", word);
    }

    // copy chars into a vector, sort and remove dups
    let mut chars: Vec<char> = pangram.chars().collect();
    chars.sort();
    chars.dedup();

    // create empty and growable string
    let mut string = String::new();
    for c in chars {
        // insert a char at the end of the string
        string.push(c);
        // insert a string at the end of the string
        string.push_str(", ");

    }

    // trimmed string is a slice to orig string, so no new allocation
    let chars_to_trim: &[char] = &[' ', ','];
    let trimmed_str: &str = string.trim_matches(chars_to_trim);
    println!("Used characters: {}", trimmed_str);

    // heap allocate a string
    let alice = String::from("I like dogs");
    // allocate new mem and store the modified string there
    let bob: String = alice.replace("dog", "cat");

    println!("Alice says: {}", alice);
    println!("Bob says: {}", bob);
}