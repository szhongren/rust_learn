// closures are also called lambdas or anonymous functions
// uses || instead of () around input variables.
// both input and return types can be inferred.
// input variable names must be specified.
// body delimination ({}) is optional for a single expression. Mandatory otherwise.
// the outer environment variables may be captured.
// calling a closure is exactly like a function: call(var).
fn main() {
    // inc via closures and functions
    fn function (i: i32) -> i32 { i + 1 }

    // annotation is similar to fn annotation but is optional, like the {} wrapping the body since it has only one expression
    // these functions are assigned to vars via let
    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred  = |i     |          i + 1  ;

    let i = 1;
    // Call the function and closures.
    println!("function: {}", function(i));
    println!("annotated closure: {}", closure_annotated(i));
    println!("inferred closure: {}", closure_inferred(i)); // the tyes of the closure args and return are inferred here

    // closure: takes no args and returns an i32
    let one = || 1;
    println!("Closure returning one: {}", one());

    // you can capture variables form the enclosing env, which functions cannot do
    let professor_x = "Charles Xavier";

    // a closure which takes no args, and returns nothing. just prints a var from the enclosing scope
    let print = || println!("Professor X's name is: {}", professor_x);

    // call the closure
    print();
}
