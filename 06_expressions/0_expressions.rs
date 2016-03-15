// rust program is mostly made up of a series of statements
// 2 most common types of statements are declaring a var binding, and using a ; with an expression
// blocks are expressions too, so they can be used as rvals, and the last expression in a block is assigned to the lval, but only if there is no ;
fn main() {
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cubed = x_squared * x;

        // this expression will be assigned to y
        x_cubed + x_squared + x
    };
    let z = {
        // the ; suppresses this expression and () is assigned to z
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}
