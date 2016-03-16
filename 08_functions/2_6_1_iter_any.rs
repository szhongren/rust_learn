// Iterator::any is a fn that when passed an Iterator, will return true if any element satisfies the predicate closure, otherwise false
fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // iter() for vecs yields &i32, destructure to i32
    println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2));
    // into_iter() for vecs yields i32, no destructuring required
    println!("2 in vec2: {}", vec2.into_iter().any(|x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // iter() for arrays yields &i32, so destructure
    println!("2 in array1; {}", array1.iter().any(|&x| x == 2));
    // into_iter() for arrays unusually also yields &i32, so destructure
    println!("2 in array2: {}", array2.into_iter().any(|&x| x == 2));
}
