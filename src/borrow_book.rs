use Library_Management_System::{Error, Library};
use std::io;

pub fn borrow_book(lib: &mut Library) -> Result<(), Error>{
    let mut id: String = String::with_capacity(2);

    println!("Write the id book you want to borrow");
    io::stdin().read_line(&mut id).map_err(|_|Error::ErrorReadBorrowBook)?;

    let id_as_number: usize = id.trim().parse().map_err(|_|Error::ErrorIDBorrowBook)?;
    let index = id_as_number - 1;

    match lib.book.get(index) {
        Some(book) => {
            if book.borrowable {
                println!(
                    "The book you are borrowing is '{}' from '{}'",
                    book.name, book.author
                );
                lib.book[index].borrowable = false;
                lib.save_in_file();
                println!("Borrow successful")
            } else {
                println!(
                    "The book '{}' from '{}' was already borrowed",
                    book.name, book.author
                );
            }
        }
        None => println!("Book not found, verify the list"),
    }
    Ok(())
}
