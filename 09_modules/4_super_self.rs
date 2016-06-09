// super and self keywords remove ambiguity when accessing items and prevent unnecessary hardcoding of paths
fn function() {
    println!("called function()");
}

mod cool {
    pub fn function() {
        println!("called cool::function()");
    }
}

mod my {
    fn function() {
        println!("called my::function()");
    }

    mod cool {
        pub fn function() {
            println!("called my::cool::function()");
        }
    }

    pub fn indirect_call() {
        // access all the functions called function from this scope
        print!("called my::indirect_call(), that \n> ");

        // self keyword refers to current module scope, so self::function and function effectively call the same thing
        self::function();
        function();

        // we can also use self to access another module inside my
        self::cool::function();

        // super keyword refers to parent scope
        super::function();

        // this binds to cool::function in the crate scope
        {
            use cool::function as root_function;
            root_function();
        }
    }
}

fn main() {
    my::indirect_call();
}
