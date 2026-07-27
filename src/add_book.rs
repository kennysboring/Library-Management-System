use library_management_system::{Error, Library, clear_terminal};
use std::io;

pub fn add_book(lib: &mut Library) -> Result<(), Error> {
    let mut name_book = String::new();
    let mut author_book = String::new();

    println!("Write the book title: ");

    io::stdin() //input no terminal
        .read_line(&mut name_book)
        .map_err(|_| Error::ErrorReadAddBook)?;

    println!("Write the book author: ");
    io::stdin() //input no terminal
        .read_line(&mut author_book)
        .map_err(|_| Error::ErrorReadAddBook)?;

    lib.add_book(name_book.trim().to_string(), author_book.trim().to_string()); //chama a função 'add_book'

    clear_terminal(); //limpa o terminal
    println!("Book added.");
    Ok(())
}
