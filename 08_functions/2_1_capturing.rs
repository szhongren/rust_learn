// Closures are inherently flexible and will do what the functionality requires to make the closure work without annotation. This allows capturing to flexibly adapt to the use case, sometimes moving and sometimes borrowing. Closures can capture variables by reference: &T, by mutable reference: &mut T, by value: T
fn main() {
    use std::mem;

    let color = "green";

    // a closure to print color which immediately borrows color and stores the borrow and closure in the print var. it will remain borrowed until print goes out of scope. println! only requires capturing by reference
    {
        let print = || println!("color: {}", color);

        print();
        print();
    }
    // print no longer exists here, and the borrowed color var is returned
    println!("color_out: {}", color);

    let mut count = 0;
    {
        // closure to inc count count take either &mut count or count but &mut count is less restrictive so it takes that. borrows count
        // mut in needed on inc because a &mut is stored inside. thus, calling the closure mutates the closure which means it needs to be mut
        let mut inc = || {
            count += 1;
            println!("count: {}", count);
        };

        inc();
        inc();
    }
    // count has still incremented here
    println!("count_out: {}", count);

    // cannot reborrow count below as it will be mutable in more than one place at a time
    // let reborrow = &mut count;


    // non-copy type
    let movable = Box::new(3);

    // mem::drop requires T so this must take by value, a copy type would copy into the closure leaving original untouched. non-copy must move so movable immediately moves into the closure
    let consume = || {
        println!("movable: {:?}", movable);
        // disposes of a value, does nothing for types which implement copy, since they are copied and then moved into the function
        mem::drop(movable);
    };

    consume(); // can only be called once as it consumes movable
}
