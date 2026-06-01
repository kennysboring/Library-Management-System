use Library_Management_System::{Error, Library, Menu};
mod add_book;
mod book_list;
mod borrow_book;
mod menu;
mod return_book;

fn main() -> Result<(), Error>{
    let mut new_library = Library::new(); //cria uma struct 'Library'  

    let mut input = [0u8; 1]; //variavel: lista mutavel que armazena 8 bits

    loop {
        menu::visual_menu();//chama a função 'visual menu'
        let choice = menu::choice_menu(&mut input);//variavel
        match choice { //escolhe qual função chamar baseado na variavel choice
            Ok(Menu::AddBook) => add_book::add_book(&mut new_library)?,
            Ok(Menu::BookList) => book_list::book_list(&mut new_library),
            Ok(Menu::BorrowBook) => borrow_book::borrow_book(&mut new_library)?,
            Ok(Menu::ReturnBook) => return_book::return_book(&mut new_library)?,
            Ok(Menu::Quit) => break,
            Err(_) => println!("Option does not exist, try again"),
        };
    }
    Ok(())
}
