use std::io::{self, Write};

#[derive(Debug)]
pub enum Error {
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

pub struct Library {
    pub book: Vec<Book>,
}

impl Library {
    pub fn new_library() -> Self {
        Library { book: Vec::new() }
    }

    pub fn add_book(&mut self, title: String, author: String) {
        let new_book = Book{
            name: title,
            author: author,
            borrowable: true,
            id: (self.book.len() + 1) as u32,
        };
        self.book.push(new_book);
    }
}

pub fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().expect("ERROR");
}
