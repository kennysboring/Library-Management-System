use library_management_system::{Error, Menu, clear_terminal};
use std::io::{self, Read};

pub fn visual_menu() {
    println!("==========Menu==========");
    println!("1. Add book");
    println!("2. Book list");
    println!("3. Borrow book");
    println!("4. Return book");
    println!("5. Quit");
}

pub fn choice_menu(input: &mut [u8; 1]) -> Result<Menu, Error> {
    let mut temp = String::with_capacity(2); //variavel; string mutavel criada com dois espaços já alocado para descartar \n

    println!("Choice the function you need: ");
    io::stdin()
        .read_exact(input)
        .map_err(|_| Error::ErrorReadMenu)?; //input no terminal que guarda só o primeiro valor digitado
    io::stdin()
        .read_line(&mut temp)
        .map_err(|_| Error::ErrorReadMenu)?; //input no terminal que guarda o resto
    temp.clear(); //limpa a variavel 'temp'

    let _ = clear_terminal(); //chama a função que limpa o terminal
    match input[0] {
        //escolhe qual opção retornar com base no valor digitado ('switch' do python
        b'1' => Ok(Menu::AddBook),
        b'2' => Ok(Menu::BookList),
        b'3' => Ok(Menu::BorrowBook),
        b'4' => Ok(Menu::ReturnBook),
        b'5' => Ok(Menu::Quit),
        _ => Err(Error::ErrorChoiceMenu),
    }
}
