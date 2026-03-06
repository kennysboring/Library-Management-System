use std::io::{self, Write};

#[derive(Debug)]
pub enum ERRO {
    ErrorReadMenu,
    
    ErrorReadAddBook,

    ErrorReadBorrowBook,
    ErrorIDBorrowBook,

    ErrorReadReturnBooK,
    ErrorIDReturnBook,
    
    ErrorChoiceMenu,
}

pub enum Menu {
    AddBook,
    BookList,
    BorrowBook,
    ReturnBook,
    Quit,
}

pub struct Book {
    pub id: u32,
    pub name: String,
    pub author: String,
    pub borrowable: bool,
}


pub fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().expect("ERROR");
}
