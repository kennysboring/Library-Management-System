use std::io::{self, Write};
use std::env;

#[derive(Debug)]
pub enum Error {
    ErrorReadMenu,

    ErrorReadAddBook,

    ErrorReadBorrowBook,
    ErrorIDBorrowBook,

    ErrorReadReturnBooK,
    ErrorIDReturnBook,

    ErrorChoiceMenu,

    ErrorClearTerminal,

    ErrorDataBase(sqlx::Error),
}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        Error::ErrorDataBase(value)
    }
}

pub enum Menu {
    AddBook,
    BookList,
    BorrowBook,
    ReturnBook,
    Quit,
}

pub enum Status {
    Available,
    AlreadyBorrowed,
    NotFound,
}

#[derive(sqlx::FromRow)]
pub struct Book {
    //informações dos livros
    pub name: String,
    pub author: String,
    pub borrowable: bool,
    pub id: i32,
}

#[derive(sqlx::FromRow)]
struct CheckBorrowable {
    borrowable: bool,
} 

pub struct Library {
    pool: sqlx::PgPool,
}

impl Library {
    pub async fn new() -> Result<Self, sqlx::Error> {
        dotenvy::dotenv().ok();
        let database_url = env::var("DATABASE_URL")
            .expect("Impossible to read DATABASE_URL into .env");
        
        let pool = sqlx::PgPool::connect(&database_url).await?;
        sqlx::migrate!("./migrations").run(&pool).await?;
        
        Ok(Library { pool })
    }

    pub async fn add_book(&self, title: String, author: String) -> Result<(), sqlx::Error>{
        sqlx::query("INSERT INTO books (name, author) VALUES ($1, $2)")
            .bind(title)
            .bind(author)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn borrow_book(&self, id: i32) -> Result<Status, sqlx::Error> {
        let status = self.verify_borrowable(id).await?;
        
        if let Status::Available = status {
            sqlx::query("UPDATE books SET borrowable = false WHERE id = $1")
                .bind(id)
                .execute(&self.pool)
                .await?;
        }

        Ok(status)
    }

    pub async fn return_book(&self, id: i32) -> Result<Status, sqlx::Error> {
        let status = self.verify_borrowable(id).await?;
        
        if let Status::AlreadyBorrowed = status {
            sqlx::query("UPDATE books SET borrowable = true WHERE id = $1")
                .bind(id)
                .execute(&self.pool)
                .await?;
        }

        Ok(status)
    }

    async fn verify_borrowable(&self, id: i32) -> Result<Status, sqlx::Error> {
        let book: Option<CheckBorrowable> = sqlx::query_as("SELECT borrowable FROM books WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;
        
        match book {
            Some(b) if b.borrowable => Ok(Status::Available),
            Some(_) => Ok(Status::AlreadyBorrowed),
            None => Ok(Status::NotFound),
        }
    }
    
    pub async fn books(&self) -> Result<Vec<Book>, sqlx::Error> {
        let books = sqlx::query_as("SELECT id, name, author, borrowable FROM books")
            .fetch_all(&self.pool)
            .await?;

        Ok(books)
    }
}

pub fn clear_terminal() -> Result<(), Error>{
    //limpa o terminal
    print!("\x1B[2J\x1B[1;1H"); //envia comandos ANSI: '\x1B[2J' limpa a tela e '\x1B[1;1H' move o cursor para o inicio
    io::stdout().flush().map_err(|_| Error::ErrorClearTerminal)?; //força a saída dos comandos
    Ok(())
}
