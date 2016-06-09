// data can be immutably borrowed any number of times
// while immutably borrowed, original data can't be berrowed.
// only 1 mutable borrow allowed at a time

struct Point { x: i32, y: i32, z: i32 }

fn main() {
    let mut point = Point { x: 0, y: 0, z: 0 };

    {
        let borrowed_point = &point;
        let another_borrow = &point;

        // data can be accessed via the refs and the original owner
        println!("Point has coords: ({}, {}, {})", borrowed_point.x, another_borrow.y, point.z);

        // can't borrow point as mut because it's currently borrowed as immutable
        // let mutable_borrow = &mut point;
        // immutable refs go out of scope
    }

    {
        let mutable_borrow = &mut point;

        // change data
        mutable_borrow.x = 5;
        mutable_borrow.y = 2;
        mutable_borrow.z = 1;

        // can't immutably borrow point because of current mutable borrow
        // let y = &point.y;

        // can't print either as println! takes an immutable refs
        // println!("point z is at {}", point.z);

        // mutable ref goes out of scope
    }

    println!("point now has coords: ({}, {}, {})", point.x, point.y, point.z);
}