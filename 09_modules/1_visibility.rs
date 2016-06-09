// modules system can be used to hierarchically split code in logical units and manage visibility between them
// by default, the items in a module have private visibility, but this can be overriden with the pub modifier. only the pub items of a module can be accessed from outside the module scope
// a module called my
mod my {
    // items in modules default to private visibility
    fn private_function() {
        println!("called my::private_function()");
    }

    // pub keyword to override default visibility
    pub fn function() {
        println!("called my::function()");
    }

    pub fn indirect_access() {
        print!("called my::indirect_access(), that\n>");
        private_function();
    }

    // modules can be nested
    pub mod nested {
        pub fn function() {
            println!("called my::nested::function()");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("called my::nested::private_function()");
        }
    }

    // mods have same rules for visibility
    mod private_nested {
        #[allow(dead_code)]
        pub fn function() {
            println!("called my::private_nested::function()");
        }
    }
}

fn function() {
    println!("called function()");
}

fn main() {
    // modules allow disambiguation between items that have the same name
    function();
    my::function();

    // Public items, including those inside nested modules, can be accessed from outside the parent module.
    my::indirect_access();
    my::nested::function();
}
