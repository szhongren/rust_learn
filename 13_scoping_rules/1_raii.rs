// rust variables hold values and also own resources
// raii stands for resource acquisition is initialization
// when an object goes out of scope its destructor is called and owned resources are freed
fn create_box() {
    // allocate an int on the heap
    let _box1 = Box::new(3i32);
    // _box1 destroyed here
}

fn main() {
    // allocate an int on the heap
    let _box2 = Box::new(5i32);

    // nested scope
    {
        let _box3 = Box::new(4i32);
        // _box3 destroyed here
    }

    // no need to manually free memory
    for _ in 0u32 .. 1_000 {
        create_box();
    }
}