// the use declaration can be used to bind a full path to a new name
// bind deeply::nested::function as other_function
use deeply::nested::function as other_function;

fn function() {
    println!("called function()");
}

mod deeply {
    pub mod nested {
        pub fn function() {
            println!("called deeply::nested::function()");
        }
    }
}

fn main() {
    // easier access to deeply::nested::function
    other_function();

    println!("entering block");
    {
        // this is eqv to use deeply::nested::function as function
        // this function() shadows the outer one
        use deeply::nested::function;
        function();

        // use bindings have local scope
        println!("leaving block!");
    }

    function();
}
