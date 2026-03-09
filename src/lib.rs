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
    pub name: String,
    pub author: String,
    pub borrowable: bool,
    pub id: u32,
}

pub struct Library {
    pub book: Vec<Book>,
    next_id: u32,
}

impl Library {
    pub fn new() -> Self {
        Library { book: Vec::new(), next_id: 0}
    }

    pub fn add_book(&mut self, title: String, author: String) {
        self.next_id += 1;

        let new_book = Book{
            name: title,
            author: author,
            borrowable: true,
            id: self.next_id,
        };
        self.book.push(new_book);
    }
}

pub fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().expect("ERROR");
}
