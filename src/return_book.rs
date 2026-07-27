use library_management_system::{Error, Library, Status};
use std::io;

pub fn return_book(lib: &mut Library) -> Result<(), Error>{
    let mut id: String = String::with_capacity(2);

    println!("Write the id book you want to return");
    io::stdin().read_line(&mut id).map_err(|_|Error::ErrorReadReturnBooK)?; //input no terminal

    let id_as_number: usize = id.trim().parse().map_err(|_|Error::ErrorIDReturnBook)?;  //trata a variavel 'id' transformando texto em número
    
    match lib.return_book(id_as_number) {
        Status::Available => {
            println!("The book was already returned");
        },
        Status::AlreadyBorrowed => {
            println!("Return successful");
        },
        Status::NotFound => println!("Book not found, verify the list"),
    }
    Ok(())
}
