use Library_Management_System::{Library};

pub fn book_list(lib: &Library){
    println!("==========BOOK LIST==========");
    if lib.book.is_empty() {
        println!("The list is empty, no one book is added before.")
    } else {
        for i in &lib.book {
            println!(
                "id:{}  |Title: {}  |Author: {}|    Borrowable: {}",
                i.id, i.name, i.author, if i.borrowable {"Yes"} else {"No"}
            );
        }
    }
}
