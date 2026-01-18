use std::io;
use crate::Book;

pub fn borrow_book(list: &mut Vec<Book>) {
    let mut id: String = String::with_capacity(2);
    let mut can_borrow: bool = false;

    println!("Write the id book you want to borrow" );
    io::stdin()
        .read_line(&mut id)
        .expect("ERROR");

    let id_as_number:usize = id.trim().parse().expect("ERROR");
    let index = id_as_number - 1;

    match list.get(index) {
        Some(book) => if book.borrowable == true {
            println!("The book you are borrowing is {} from {}", book.name, book.author);
            can_borrow = true;
        } else {
            println!("The book {} from {} was already borrowed", book.name, book.author);
        },
        None => println!("Book not found, verify the list"),
    }
    
    if can_borrow == true {
        list[index].borrowable = false;
        println!("Borrow successful")
    }
}