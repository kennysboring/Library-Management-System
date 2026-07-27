use library_management_system::Library;

pub fn book_list(lib: &Library){
    println!("==========BOOK LIST==========");
    if lib.book.is_empty() { //verifica se a varivel 'book' da struct 'Library' esta vazia
        println!("The list is empty, no one book is added before.")
    } else {
        for i in &lib.book { //passa por cada 'book' da struct 'Library' e printa no terminal
            println!(
                "id:{}  |Title: {}  |Author: {}|    Borrowable: {}",
                i.id, i.name, i.author, if i.borrowable {"Yes"} else {"No"}
            );
        }
    }
}
