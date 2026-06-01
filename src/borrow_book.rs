use Library_Management_System::{Error, Library};
use std::io;

pub fn borrow_book(lib: &mut Library) -> Result<(), Error>{
    let mut id: String = String::with_capacity(2);

    println!("Write the id book you want to borrow");
    io::stdin().read_line(&mut id).map_err(|_|Error::ErrorReadBorrowBook)?; //input no terminal

    let id_as_number: usize = id.trim().parse().map_err(|_|Error::ErrorIDBorrowBook)?; //trata a variavel 'id' transformando texto em número
    let index = id_as_number - 1; //subtrai 1 porque o indice começa em '0'

    match lib.book.get(index) { //lida com o erro caso não tenha nenhum 'book' na struct 'Library'
        Some(book) => {
            if book.borrowable { //caso 'borrowable = true'
                println!(
                    "The book you are borrowing is '{}' from '{}'",
                    book.name, book.author
                );
                lib.book[index].borrowable = false; //troca 'borrowable' para false
                lib.save_in_file(); //troca o estado armazenado no arquivo 'books.txt' (banco de dados)
                println!("Borrow successful")
            } else { //caso borrowable = false
                println!(
                    "The book '{}' from '{}' was already borrowed",
                    book.name, book.author
                );
            }
        }
        None => println!("Book not found, verify the list"), //'book' vazio
    }
    Ok(())
}
