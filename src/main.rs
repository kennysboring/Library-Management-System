mod return_book;
mod borrow_book;
mod add_book;
mod book_list;
mod menu;
pub enum Menu {
    AddBook,
    BookList,
    BorrowBook,
    ReturnBook,
    Quit,
    Error,
}

pub struct Book {
    name: String,
    author: String,
    borrowable: bool,
    id: u32
}

fn main() {
    let mut library: Vec<Book> = Vec::new();
    let mut input = [0u8; 1];
    let mut temp = String::with_capacity(2);

    loop {
        menu::visual_menu();
        let choice = menu::choice_menu(&mut input, &mut temp);
        match choice {
            Menu::AddBook => add_book::add_book(&mut library),
            Menu::BookList => book_list::book_list(&mut library),
            Menu::BorrowBook => borrow_book::borrow_book(&mut library),
            Menu::ReturnBook => return_book::return_book(&mut library),
            Menu::Quit => break,
            Menu::Error => println!("ERROR 404"),
        }
    }   
}
