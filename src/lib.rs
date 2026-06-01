use std::io::{self, Write, BufRead};
use std::fs::File;
use std::path::Path;

#[derive(Debug)]
pub enum Error { 
    ErrorReadMenu,

    ErrorReadAddBook,

    ErrorReadBorrowBook,
    ErrorIDBorrowBook,

    ErrorReadReturnBooK,
    ErrorIDReturnBook,

    ErrorChoiceMenu,
}

pub enum Menu {
    AddBook,
    BookList,
    BorrowBook,
    ReturnBook,
    Quit,
}

pub struct Book { //informações dos livros
    pub name: String,
    pub author: String,
    pub borrowable: bool,
    pub id: u32,
}

pub struct Library {
    pub book: Vec<Book>, //lista dos livros
    next_id: u32, //armazena o último ID adicionado para garantir que cada novo livro receba um ID único
}

const FILE_PATH: &str = "src/books.txt";

impl Library {
    pub fn new() -> Self { //cria uma 'Library' nova
        let mut lib = Library {
            book: Vec::new(),
            next_id: 0,
        };
        lib.load_from_file();//carrega o arquivo 'book.txt' (banco de dados)
        lib
    }

    pub fn add_book(&mut self, title: String, author: String) { //adiciona um livro
        self.next_id += 1; 

        let new_book = Book { //variavel com as informações dos livros
            name: title,
            author: author,
            borrowable: true,
            id: self.next_id,
        };
        self.book.push(new_book); //adiciona a variavel na lista 'book'
        self.save_in_file(); //salva a lista no arquivo
    }

    fn book_to_line(book: &Book) -> String { //formata as informações do livro em uma linha e retorna ela
        format!(
            "{};{};{};{}\n",
            book.id, book.name, book.author,
            if book.borrowable { "true" } else { "false" }
        )
    }

    pub fn save_in_file(&self) {
        //cria o arquivo 'book.txt'
        let mut file = File::create(FILE_PATH)
            .expect("ERROR to create/open a file books.txt");

        for book in &self.book { //passa por cada livro e adiciona em 'books.txt.'  
            let line = Self::book_to_line(book); //variavel: armazena as infromações dos livros formatada
            file.write_all(line.as_bytes()) //escreve a variavel no arquivo
                .expect("ERROR WRITE");
        }
    }

    fn load_from_file(&mut self) {
        if !Path::new(FILE_PATH).exists() { //sai da função se o arquivo não existe
            return; 
        }

        let file = File::open(FILE_PATH) //carrega o arquivo
            .expect("ERROR to open books.txt");

        let reader = io::BufReader::new(file); //salva as informações do arquivo da RAM

        for line in reader.lines() { //passa linha por linha do arquivo
            let line = line.expect("ERROR to read a line file"); //lida com o erro caso tenha algum problema na linha
            if line.trim().is_empty() { //verifica se a linha esta vazia
                continue; //pula para o proximo valor de 'line'
            }

            let parts: Vec<&str> = line.splitn(4, ';').collect();//armazena cada informação da linha em um indice da lista
            if parts.len() != 4 { //lida com erros caso tenha mais de 4 partes
                println!("Line invaliable: {}", line);
                continue; //pula para o proximo valor de 'line'
            }

            let id: u32 = match parts[0].trim().parse() { //transforma o primeiro valor em um número e trata o erro
                Ok(v) => v,
                Err(_) => {
                    println!("ID invaliable: {}", parts[0]);
                    continue; //pula para o proximo valor de 'line'
                }
            };

            let book = Book { //salva as informações da linha em uma variavel
                id,
                name: parts[1].trim().to_string(),
                author: parts[2].trim().to_string(),
                borrowable: parts[3].trim() == "true",
            };

            if id >= self.next_id { //faz a verificão do id para o quando for adicionar o proximo livro
                self.next_id = id;
            }

            self.book.push(book); //adiciona a variavel na lista da struct 'Library'
        }
    }
}

pub fn clear_terminal() { //limpa o terminal
    print!("\x1B[2J\x1B[1;1H"); //envia comandos ANSI: '\x1B[2J' limpa a tela e '\x1B[1;1H' move o cursor para o inicio
    io::stdout().flush().expect("ERROR"); //força a saída dos comandos
}