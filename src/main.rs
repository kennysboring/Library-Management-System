use Library_Management_System::{ERRO, Book, Menu};
mod add_book;
mod book_list;
mod borrow_book;
mod menu;
mod return_book;

fn main() -> Result<(), ERRO>{
    let mut library: Vec<Book> = Vec::new();
    let mut input = [0u8; 1];
    let mut temp = String::with_capacity(2);

    loop {
        menu::visual_menu();
        let choice = menu::choice_menu(&mut input, &mut temp);
        match choice {
            Ok(Menu::AddBook) => add_book::add_book(&mut library)?,
            Ok(Menu::BookList) => book_list::book_list(&mut library),
            Ok(Menu::BorrowBook) => borrow_book::borrow_book(&mut library)?,
            Ok(Menu::ReturnBook) => return_book::return_book(&mut library)?,
            Ok(Menu::Quit) => break,
            Err(_) => return Err(ERRO::ErrorChoiceMenu),
        };
    }
    Ok(())
}
