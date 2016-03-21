// to link a crate to the previously created lib, the extern crate declaration must be used. this will link the library and also import all its items under a module named the same as the library. same visibility rules as modules apply
// link to lib, import items under the rary module
extern crate rary;

fn main() {
    rary::public_function();

    rary::indirect_access();
}


// call rust -L . executable.rs && ,/executable to add curr dir to the lib search path
