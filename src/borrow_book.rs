use Library_Management_System::{ERRO, Book};
use std::io;

pub fn borrow_book(list: &mut Vec<Book>) -> Result<(), ERRO>{
    let mut id: String = String::with_capacity(2);

    println!("Write the id book you want to borrow");
    io::stdin().read_line(&mut id).map_err(|_|ERRO::ErrorReadBorrowBook)?;

    let id_as_number: usize = id.trim().parse().map_err(|_|ERRO::ErrorIDBorrowBook)?;
    let index = id_as_number - 1;

    match list.get(index) {
        Some(book) => {
            if book.borrowable {
                println!(
                    "The book you are borrowing is {} from {}",
                    book.name, book.author
                );
                list[index].borrowable = false;
                println!("Borrow successful")
            } else {
                println!(
                    "The book {} from {} was already borrowed",
                    book.name, book.author
                );
            }
        }
        None => println!("Book not found, verify the list"),
    }
    Ok(())
}
