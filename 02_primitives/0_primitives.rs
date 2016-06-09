fn main() {
    // vars can be type annotated
    let logical: bool = true;

    let a_float: f64 = 1.0; // regular annotation
    let an_integer = 5i32; // suffix annotation

    // defaults are f64 or i32
    let default_float = 3.0;
    let default_integer = 7;

    let mut mutable = 12;

    // cannot change the type of a var
}

/* types incl
 * signed integers: i8, i16, i32, i64 and isize (pointer size)
 * unsigned integers: u8, u16, u32, u64 and usize (pointer size)
 * floating point: f32, f64
 * char Unicode scalar values like 'a', 'α' and '∞' (4 bytes each)
 * bool either true or false
 * and the unit type (), whose only value is also ()
 * arrays like [1, 2, 3]
 * tuples like (1, true)
 */
