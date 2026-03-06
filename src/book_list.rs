use Library_Management_System::Book;

pub fn book_list(list: &mut Vec<Book>){
    println!("==========BOOK LIST==========");
    if list.is_empty() {
        println!("The list is empty, no one book is added before.")
    } else {
        for i in list {
            println!(
                "id:{}-
            Title: {} 
            Author: {}
            Borrowable: {}",
                i.id, i.name, i.author, i.borrowable
            );
        }
    }
}
