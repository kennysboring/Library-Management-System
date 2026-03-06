use Library_Management_System::{clear_terminal, ERRO, Book};
use std::io;

pub fn add_book(list: &mut Vec<Book>) -> Result<(), ERRO> {
    let mut name_book = String::new();
    let mut author_book = String::new();

    println!("Write the book title: ");
    io::stdin()
        .read_line(&mut name_book)
        .map_err(|_|ERRO::ErrorReadAddBook)?;

    println!("Write the book author: ");
    io::stdin()
        .read_line(&mut author_book)
        .map_err(|_|ERRO::ErrorReadAddBook)?;

    list.push(Book {
        name: name_book.trim().to_string(),
        author: author_book.trim().to_string(),
        borrowable: true,
        id: (list.len() + 1) as u32,
    });

    clear_terminal();
    println!("Book added.");
    Ok(())
}
