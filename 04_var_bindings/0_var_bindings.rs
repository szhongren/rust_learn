// rust provides type safety via static typing, and var bindings can be annotated when declared
// in most cases, the compiler can infer the type of the var from the context
fn main() {
    let an_int = 1u32;
    let a_bool = true;
    let unit = ();

    // copy an_int into copied_int
    let copied_int = an_int;

    println!("an int: {:?}", copied_int);
    println!("a bool: {:?}", a_bool);
    println!("meet the unit val: {:?}", unit);

    // compiler warns about unused var bindings, but can be silenced with an underscore prefix
    let _unused_var = 3u32;

    let noisy_unused_var = 2u32;
}
