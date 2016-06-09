#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Book {
    // &'static str refers to a str allocated in read only memory
    author: &'static str,
    title: &'static str,
    year: u32,
}

// this fn takes a ref to a book
fn borrow_book(book: &Book) {
    println!("I immutably borrowed {} - {} edition", book.title, book.year);
}

// this fn takes a ref to a mutable book and changes year to 2014
fn new_edition(book: &mut Book) {
    book.year = 2014;
    println!("I mutably borrowed {} - {} edition", book.title, book.year);
}

fn main() {
    // create immutable book called immutabook
    let immutabook = Book {
        author: "Douglas Hofstadter",
        title: "Gödel, Escher, Bach",
        year: 1979,
    };

    let mut mutabook = immutabook;

    borrow_book(&immutabook);
    borrow_book(&mutabook);
    new_edition(&mut mutabook);
    // error! cannot borrow immutable obj as mutable
    // new_edition(&mut immutabook);
}