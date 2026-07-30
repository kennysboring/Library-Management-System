use std::println;

use library_management_system::{Library, Menu};
mod add_book;
mod book_list;
mod borrow_book;
mod menu;
mod return_book;

#[tokio::main]
async fn main() {
    let new_library = Library::new().await.expect("Error to inicializate struct Library"); //cria uma struct 'Library'  

    let mut input = [0u8; 1]; //variavel: lista mutavel que armazena 8 bits

    loop {
        menu::visual_menu(); //chama a função 'visual menu'
        let choice = menu::choice_menu(&mut input); //variavel
        match choice {
            //escolhe qual função chamar baseado na variavel choice
            Ok(Menu::AddBook) => {
                if let Err(e) = add_book::add_book(&new_library).await {
                    println!("Error {:?}", e);
                }
            }
            Ok(Menu::BookList) => {
                if let Err(e) = book_list::book_list(&new_library).await {
                    println!("Error {:?}", e);
                }
            }
            Ok(Menu::BorrowBook) => {
                if let Err(e) = borrow_book::borrow_book(&new_library).await {
                    println!("Error {:?}", e);
                }
            }
            Ok(Menu::ReturnBook) => {
                if let Err(e) = return_book::return_book(&new_library).await {
                    println!("Error {:?}", e);
                }
            }
            Ok(Menu::Quit) => break,
            Err(_) => println!("Option does not exist, try again"),
        };
    }
}
