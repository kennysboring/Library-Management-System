use crate::Book;

pub fn book_list(list: &mut Vec<Book>) {
    let mut num = 1;

    println!("==========BOOK LIST==========");
    if list.is_empty() {
        println!("The list is empty, no one book is added before.")
    } else {for i in list {
        println!("{}- Title: {} | Author: {}", num, i.name, i.author);
        num = num + 1;
    }
}
}