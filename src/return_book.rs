use std::io;
use crate::Book;

pub fn return_book(list: &mut Vec<Book>) {
    let mut id: String = String::with_capacity(2);
    let mut can_return: bool = false;

    println!("Write the id book you want to return" );
    io::stdin()
        .read_line(&mut id)
        .expect("ERROR");

    let id_as_number:usize = id.trim().parse().expect("ERROR");
    let index = id_as_number - 1;

    match list.get(index) {
        Some(book) => if book.borrowable == false {
            println!("The book you are returning is {} from {}", book.name, book.author);
            can_return = true;
        } else {
            println!("The book {} from {} was already returned", book.name, book.author);
        },
        None => println!("Book not found, verify the list"),
    }
    
    if can_return == true {
        list[index].borrowable = true;
        println!("Return successful")
    }
}