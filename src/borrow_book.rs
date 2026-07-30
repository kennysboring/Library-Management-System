use library_management_system::{Error, Library, Status};
use std::io;

pub async fn borrow_book(lib: &Library) -> Result<(), Error> {
    let mut id: String = String::with_capacity(2);

    println!("Write the id book you want to borrow");
    io::stdin()
        .read_line(&mut id)
        .map_err(|_| Error::ErrorReadBorrowBook)?; //input no terminal

    let id_as_number: i32 = id.trim().parse().map_err(|_| Error::ErrorIDBorrowBook)?; //trata a variavel 'id' transformando texto em número

    match lib.borrow_book(id_as_number).await? {
        Status::Available => {
            println!("Borrow successful");
        }
        Status::AlreadyBorrowed => {
            println!("The book was already borrowed");
        }
        Status::NotFound => println!("Book not found, verify the list"),
    }
    Ok(())
}
