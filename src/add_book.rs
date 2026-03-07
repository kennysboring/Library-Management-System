use Library_Management_System::{Error, Library, clear_terminal};
use std::io;

pub fn add_book(lib: &mut Library) -> Result<(), Error> {
    let mut name_book = String::new();
    let mut author_book = String::new();

    println!("Write the book title: ");
    io::stdin()
        .read_line(&mut name_book)
        .map_err(|_|Error::ErrorReadAddBook)?;

    println!("Write the book author: ");
    io::stdin()
        .read_line(&mut author_book)
        .map_err(|_|Error::ErrorReadAddBook)?;

    lib.add_book(name_book.trim().to_string(), author_book.trim().to_string());

    clear_terminal();
    println!("Book added.");
    Ok(())
}
