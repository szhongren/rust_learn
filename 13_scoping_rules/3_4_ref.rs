// when doing pattern matching or destruct with let, ref keyword can be used to take refs to the fields of a struct
#[derive(Clone, Copy)]
struct Point{ x: i32, y: i32 }

fn main() {
    let c = 'Q';

    // a ref borrow on the left of an assignment is eqv to a & borrow on the right side
    let ref ref_c1 = c;
    let ref_c2 = &c;

    println!("ref_c1 equals ref_c2: {}", *ref_c1 == *ref_c2);

    let point = Point{ x: 0, y: 0 };

    // ref is also valid when destructing a struct
    let _copy_of_x = {
        // ref_to_x is a ref to the x field of point
        let Point { x: ref ref_to_x, y: _ } = point;

        *ref_to_x
    };

    // mutable copy of point
    let mut mutable_point = point;

    {
        // ref can be paired with mut to take mutable refs
        let Point { x: _, y: ref mut mut_ref_to_y } = mutable_point;

        // mutate the y field of mutable_point via a mutable ref
        *mut_ref_to_y = 1;
    }

    println!("point is ({}, {})", point.x, point.y);
    println!("mutable_point is ({}, {})", mutable_point.x, mutable_point.y);

    // a mutable tuple that includes a pointer
    let mut mutable_tuple = (Box::new(5u32), 3u32);

    {
        // destruct mutable_tuple to change the value of last
        let (_, ref mut last) = mutable_tuple;
        *last = 2u32;
    }

    println!("tuple is {:?}", mutable_tuple);
}