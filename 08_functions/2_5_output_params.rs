// returning closures as output params is also possible, but right now rust only supports returning concrete types, and closures are generic. to make it concrete and returnable, we need to box it
// valid traits only differ for FnOnce, so the unstable FnBox type is needed right now
// also, the move keyword must be used, making all captures by value. any captures by reference would be dropped when the function exits, which would leave invalid references in the closure
fn create_fn() -> Box<Fn()> {
    let text = "Fn".to_owned();
    // return the closure specified below in a Box
    Box::new(move || println!("This is a: {}.", text))
}

fn create_fnmut() -> Box<FnMut()> {
    let text = "FnMut".to_owned();

    Box::new(move || println!("This is a: {}.", text))
}

fn main() {
    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();

    fn_plain();
    fn_mut();
}
