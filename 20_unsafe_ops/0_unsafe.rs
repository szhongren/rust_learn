// unsafe blocks are used for mainly:
// - dereferencing raw pointers
// - calling a function over FFI
// - changing types with std::cast::transmute
// - inline assembly
fn main() {
    // dereferencing a raw pointer can only be done in an unsafe block
    let raw_p: *const u32 = &10;

    unsafe {
        assert!(*raw_p == 10);
    }

    // transmutes allow conversion from 1 type to another, but must have same size and alignment
    let u: &[u8] = &[49, 50, 51];

    unsafe {
        assert!(u == std::mem::transmute::<&str, &[u8]>("123"));
    }
}