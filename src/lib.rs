use std::io::{self, Write, BufRead};
use std::fs::{File, OpenOptions};
use std::path::Path;

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

const FILE_PATH: &str = "src/books.txt";

impl Library {
    pub fn new() -> Self {
        let mut lib = Library {
            book: Vec::new(),
            next_id: 0,
        };
        lib.load_from_file();
        lib
    }

    pub fn add_book(&mut self, title: String, author: String) {
        self.next_id += 1;

        let new_book = Book {
            name: title,
            author: author,
            borrowable: true,
            id: self.next_id,
        };
        self.book.push(new_book);
        self.save_in_file();
    }

    fn book_to_line(book: &Book) -> String {
        format!(
            "{};{};{};{}\n",
            book.id, book.name, book.author,
            if book.borrowable { "true" } else { "false" }
        )
    }

    pub fn save_in_file(&self) {
        let mut file = File::create(FILE_PATH)
            .expect("ERROR to create/open a file books.txt");

        for book in &self.book {
            let line = Self::book_to_line(book);
            file.write_all(line.as_bytes())
                .expect("ERROR WRITE");
        }
    }

    fn load_from_file(&mut self) {
        if !Path::new(FILE_PATH).exists() {
            return; 
        }

        let file = File::open(FILE_PATH)
            .expect("ERROR to open books.txt");

        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            let line = line.expect("ERROR to read a line file");
            if line.trim().is_empty() {
                continue;
            }

            let parts: Vec<&str> = line.splitn(4, ';').collect();
            if parts.len() != 4 {
                println!("Line invaliable: {}", line);
                continue;
            }

            let id: u32 = match parts[0].trim().parse() {
                Ok(v) => v,
                Err(_) => {
                    println!("ID invaliable: {}", parts[0]);
                    continue;
                }
            };

            let book = Book {
                id,
                name: parts[1].trim().to_string(),
                author: parts[2].trim().to_string(),
                borrowable: parts[3].trim() == "true",
            };

            if id >= self.next_id {
                self.next_id = id;
            }

            self.book.push(book);
        }
    }
}

pub fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().expect("ERROR");
}