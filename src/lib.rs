use std::env;
use std::io::{self, Write};

#[derive(Debug)]
pub enum Error {
    ErrorReadMenu,

    ErrorReadAddBook,
    EmptySpaceAddBook,

    ErrorReadBorrowBook,
    ErrorIDBorrowBook,

    ErrorReadReturnBooK,
    ErrorIDReturnBook,

    ErrorChoiceMenu,

    ErrorClearTerminal,

    ErrorDatabase(sqlx::Error),
}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        Error::ErrorDatabase(value)
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
        let database_url =
            env::var("DATABASE_URL").expect("Impossible to read DATABASE_URL into .env");

        let pool = sqlx::PgPool::connect(&database_url).await?;
        sqlx::migrate!("./migrations").run(&pool).await?;

        Ok(Library { pool })
    }

    pub async fn add_book(&self, title: String, author: String) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO books (name, author) VALUES ($1, $2)")
            .bind(title)
            .bind(author)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn borrow_book(&self, id: i32) -> Result<Status, sqlx::Error> {
        let result = sqlx::query_as::<_, CheckBorrowable>(
            "UPDATE books SET borrowable = false WHERE id = $1 AND borrowable = true RETURNING borrowable"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
    
        if result.is_some() {
            return Ok(Status::Available); // emprestou com sucesso
        }
    
        // não emprestou - precisa descobrir se é porque não existe ou porque já estava emprestado
        let exists = self.verify_borrowable(id).await?;
        Ok(match exists {
            Status::NotFound => Status::NotFound,
            _ => Status::AlreadyBorrowed,
        })
    }

    pub async fn return_book(&self, id: i32) -> Result<Status, sqlx::Error> {
        let result = sqlx::query_as::<_, CheckBorrowable>(
            "UPDATE books SET borrowable = true WHERE id = $1 AND borrowable = false RETURNING borrowable"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
    
        if result.is_some() {
            return Ok(Status::AlreadyBorrowed); // retornou com sucesso
        }
    
        // não retornou, precisa descobrir se é porque não existe ou porque já foi retornado
        let exists = self.verify_borrowable(id).await?;
        Ok(match exists {
            Status::NotFound => Status::NotFound,
            _ => Status::Available,
        })
    }

    async fn verify_borrowable(&self, id: i32) -> Result<Status, sqlx::Error> {
        let book: Option<CheckBorrowable> =
            sqlx::query_as("SELECT borrowable FROM books WHERE id = $1")
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

pub fn clear_terminal() -> Result<(), Error> {
    //limpa o terminal
    print!("\x1B[2J\x1B[1;1H"); //envia comandos ANSI: '\x1B[2J' limpa a tela e '\x1B[1;1H' move o cursor para o inicio
    io::stdout()
        .flush()
        .map_err(|_| Error::ErrorClearTerminal)?; //força a saída dos comandos
    Ok(())
}
