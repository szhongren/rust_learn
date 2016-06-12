// a HashSet is just a HashMap where we just care about the keys, where all are unique
// sets have 4 primary operations:
// - union
// - difference, items in first but not second
// - intersection
// - symmetric_difference, all the elements in either set but not both
use std::collections::HashSet;

fn main() {
    let mut a: HashSet<i32> = vec!(1i32, 2, 3).into_iter().collect();
    let mut b: HashSet<i32> = vec!(2i32, 3, 4).into_iter().collect();

    assert!(a.insert(4));
    assert!(b.contains(&4));

    // HashSet::insert() returns false if there was a value already present
    // assert!(b.insert(4), "Value 4 in already in set b!");
    b.insert(5);

    // if a collection's element type implements Debug, then the collection implements Debug
    println!("A: {:?}", a);
    println!("B: {:?}", b);

    // print [1, 2, 3, 4, 5] in arbitrary order
    println!("Union: {:?}", a.union(&b).collect::<Vec<&i32>>());

    // this should print [1]
    println!("Difference: {:?}", a.difference(&b).collect::<Vec<&i32>>());

    // print [2, 3, 4] in arbitrary order
    println!("Intersection: {:?}", a.intersection(&b).collect::<Vec<&i32>>());

    // print [1. 5]
    println!("Symmetric difference: {:?}", a.symmetric_difference(&b).collect::<Vec<&i32>>());
}