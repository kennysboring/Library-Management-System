use std::io;
use Library_Management_System::clear_terminal;

use crate::Book;

pub fn add_book(list: &mut Vec<Book>){
    let mut name_book = String::new();
    let mut author_book = String::new();

    println!("Write the book title: ");
    io::stdin()
        .read_line(&mut name_book)
        .expect("Error, write the name again.");

    println!("Write the book author: ");
    io::stdin()
        .read_line(&mut author_book)
        .expect("Errot, write the author again.");
  
    list.push(Book{
                name: name_book.trim().to_string(),
                author: author_book.trim().to_string(),
            });
    
    clear_terminal();
    println!("Book added.")
}