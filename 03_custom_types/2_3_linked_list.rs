// common use for enums is to create a linked list
use List::{ Cons, Nil };

#[derive(Debug)]
enum List {
    // Cons: tuple struct that wraps an element and a pointer to the next node
    // Box<type> means that the type is heap allocated and not stack allocated, and is a smart pointer to a heap of the type
    // when a box goes out of scope, its destructor is called, the inner object is destroyed, and the memory is the heap is freed
    // boxes can be dereferenced with *, like c-pointers
    Cons(u32, Box<List>),
    // Nil:: a node that signifies the end of the linked list
    Nil,
}

// methods con be attached to an enum (or struct) like class methods with impl
impl List {
    // create an empty list
    fn new() -> List {
        // Nil has type list
        Nil
    }

    // consume a list, and return the same list with a new element at its front
    fn prepend(self, elem: u32) -> List {
        // cons also has type list
        Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        // self has to be matched, because the behavior of this method depends on the variant of self
        // self has type &List (reference to List) and *self has type List, and matching on a concrete type T is preferred over a match on a reference &T
        match *self {
            // can't take ownership of the tail, because self is borrowed. instead we take a reference to the tail
            Cons(_, ref tail) => 1 + tail.len(), // recursive match
            // use dot operator for references too, unlike -> in C
            Nil => 0 // return value
        }
    }

    // return representation of the list as a heap allocated string
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                // format! is similar to print!, but returns a heap allocated string instead of printing to the console
                format!("{}, {}", head, tail.stringify()) // returns a formatted string
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}

fn main() {
    // create an empty linked list, that has to be mutable because we are prepending
    let mut list = List::new();

    // append some elements
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // show the final state of the list
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
    println!("{:?}", list);
}
