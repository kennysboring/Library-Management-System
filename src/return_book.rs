use Library_Management_System::{Error, Library};
use std::io;

pub fn return_book(lib: &mut Library) -> Result<(), Error>{
    let mut id: String = String::with_capacity(2);

    println!("Write the id book you want to return");
    io::stdin().read_line(&mut id).map_err(|_|Error::ErrorReadReturnBooK)?; //input no terminal

    let id_as_number: usize = id.trim().parse().map_err(|_|Error::ErrorIDReturnBook)?;  //trata a variavel 'id' transformando texto em número
    let index = id_as_number - 1; //subtrai 1 porque o indice começa em '0'

    match lib.book.get(index) { //lida com o erro caso não tenha nenhum 'book' na struct 'Library'
        Some(book) => {
            if !book.borrowable { //caso 'borrowable = false'
                println!(
                    "The book you are returning is '{}' from '{}'",
                    book.name, book.author
                );
                lib.book[index].borrowable = true; //troca 'borrowable' para false
                lib.save_in_file(); //troca o estado armazenado no arquivo 'books.txt' (banco de dados)
                println!("Return successful")
            } else { //caso 'borrowable = true'
                println!(
                    "The book '{}' from '{}' was already returned",
                    book.name, book.author
                );
            }
        }
        None => println!("Book not found, verify the list"), //'book' vazio
    }
    Ok(())
}
