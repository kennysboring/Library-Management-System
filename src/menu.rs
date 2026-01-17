use std::io::{self, Read};
use Library_Management_System::clear_terminal; 
use crate::Menu;


pub fn visual_menu() {
    println!("==========Menu==========");
    println!("1. Add book");
    println!("2. Book list");
    println!("3. Borrow book");
    println!("4. Return book");
    println!("5. Quit");
}

pub fn choice_menu(input:&mut [u8; 1], temp: &mut String) -> Menu {

    println!("Choice the function you need: ");
    io::stdin()
        .read_exact(input)
        .expect("ERROR");
    io::stdin()
        .read_line(temp)
        .expect("ERROR");
    temp.clear();

    clear_terminal();
    match input[0] {
        b'1' => Menu::AddBook,
        b'2' => Menu::BookList,
        b'3' => Menu::BorrowBook,
        b'4' => Menu::ReturnBook,
        b'5' => Menu::Quit,
        _ => Menu::Error,

    }
    
}