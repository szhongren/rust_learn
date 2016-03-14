use std::fmt;

// define a List containing a Vec
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // deref self and create a ref to vec via destructuring
        let List(ref vec) = *self;

        // try macro handles all the errors that might be thrown
        try!(write!(f, "["));

        // iterate over vec in v while enumerating the iteration count in count
        for (count, v) in vec.iter().enumerate() {
            // for every elem except the first add a comma before calling write!
            if count != 0 {try!(write!(f, ", "));}
            try!(write!(f, "{}", v));
        }
        // returning value
        write!(f, "]")
    }
}

fn main() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}
