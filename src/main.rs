mod add_book;
enum Menu {
    AddBook,
    BookList,
    BorrowBook,
    ReturnBook,
    Quit,
}

pub struct Book {
    name: String,
    author: String,
}

fn visual_menu() {
    println!("==========Menu==========");
    println!("1. Add book");
    println!("1. Book list");
    println!("3. Borrow book");
    println!("4. Return book");
    println!("5. Quit");
}
fn main() {
    let mut library: Vec<Book> = Vec::new();
    let choice:Menu = Menu::AddBook;

    loop {
        visual_menu();
        match choice {
            Menu::AddBook => add_book::add_book(&mut library),
            Menu::BookList => {},
            Menu::BorrowBook => {},
            Menu::ReturnBook => {},
            Menu::Quit => break,
        }
    }   
}
