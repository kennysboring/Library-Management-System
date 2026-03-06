use Library_Management_System::{clear_terminal, ERRO, Menu};
use std::io::{self, Read};

pub fn visual_menu() {
    println!("==========Menu==========");
    println!("1. Add book");
    println!("2. Book list");
    println!("3. Borrow book");
    println!("4. Return book");
    println!("5. Quit");
}

pub fn choice_menu(input: &mut [u8; 1], temp: &mut String) -> Result<Menu, ERRO>{
    println!("Choice the function you need: ");
    io::stdin().read_exact(input).map_err(|_|ERRO::ErrorReadMenu)?;
    io::stdin().read_line(temp).map_err(|_|ERRO::ErrorReadMenu)?;
    temp.clear();

    clear_terminal();
    match input[0] {
        b'1' => Ok(Menu::AddBook),
        b'2' => Ok(Menu::BookList),
        b'3' => Ok(Menu::BorrowBook),
        b'4' => Ok(Menu::ReturnBook),
        b'5' => Ok(Menu::Quit),
        _ => Err(ERRO::ErrorChoiceMenu),
    }
}
