mod add_book;
enum Menu {
    AddBook,
    BookList,
    BorrowBook,
    ReturnBook,
}

pub struct Book {
    name: String,
    author: String,
}

fn main() {
    let mut library: Vec<Book> = Vec::new();

    let choice:Menu = Menu::AddBook;
    match choice {
        Menu::AddBook => {},
        Menu::BookList => {},
        Menu::BorrowBook => {},
        Menu::ReturnBook => {},
    }
}
