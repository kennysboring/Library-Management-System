use library_management_system::Library;

pub async fn book_list(lib: &Library) -> Result<(), sqlx::Error>{

    let list_all_books = lib.books().await?;
    println!("==========BOOK LIST==========");
    if list_all_books.is_empty() {
        //verifica se a varivel 'book' da struct 'Library' esta vazia
        println!("The list is empty, no one book is added before.")
    } else {
        for i in list_all_books {
            //passa por cada 'book' da struct 'Library' e printa no terminal
            println!(
                "id:{}  |Title: {}  |Author: {}|    Borrowable: {}",
                i.id,
                i.name,
                i.author,
                if i.borrowable { "Yes" } else { "No" }
            );
        }
    }
    Ok(())
}
