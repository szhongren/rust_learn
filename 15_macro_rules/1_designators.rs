// the args of a macro are prefixed by a $ and type annotated with a designator, which is one o the following:
// block
// expr: expression
// ident: var/fn names
// item
// pat: pattern
// path
// stmt: statement
// tt: token tree
// ty: type
macro_rules! create_function {
    // func_name is an identity, and we replace the below code where this macro is called
    ($func_name: ident) => (
        fn $func_name() {
            // stringify converts ident into a string
            println!("You called {:?}()", stringify!($func_name));
        }
    )
}

// create fns called foo and bar with above macro
create_function!(foo);
create_function!(bar);

macro_rules! print_result {
    // this macro takes an exp of type expr and prints it as a string along with its result
    ($exp: expr) => (
        println!("{:?} = {:?}", stringify!($exp), $exp);
    )
}

fn main() {
    foo();
    bar();

    print_result!(1u32 + 1);

    // blocks are expressions too!
    print_result!({
        let x = 1u32;

        x * x + 2 * x - 1
    });
}
