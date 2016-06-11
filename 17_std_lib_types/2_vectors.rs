// vecs are resizable arrays, represented with 3 words: ptr, len, and capacity
fn main() {
    // iterators can be collected into vecs
    let collected_iterator: Vec<i32> = (0..10).collect();
    println!("Collected (0..10) into: {:?}", collected_iterator);

    // the vec! macro can init a vector
    let mut xs = vec![1i32, 2, 3];
    println!("Initial vector: {:?}", xs);

    // insert new element at the end of vector
    println!("Push 4 into the vector");
    xs.push(4);
    println!("Vector: {:?}", xs);

    println!("Vector Size: {}", xs.len());

    println!("Second element: {}", xs[1]);

    println!("Pop last element: {:?}", xs.pop());

    println!("Fourth element: {}", xs[3]); // will panic

}