# Library Management System

A Rust CLI for managing book loans in a library, with PostgreSQL persistence.

## Stack

- **Rust** + **Tokio** (async runtime)
- **PostgreSQL** via **sqlx** (async queries, migrations)
- **Docker Compose** to run the database locally

## Features

- Add a book (title + author)
- List all books and their status (available / borrowed)
- Borrow a book by ID
- Return a book by ID

## Prerequisites

- [Docker](https://docs.docker.com/get-docker/) and Docker Compose
- [Rust](https://www.rust-lang.org/tools/install) (via `rustup`)
- [`sqlx-cli`](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli) (optional — only needed if you want to run migrations manually; the binary already applies pending migrations automatically on startup)

## How to run

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd library_management_system
   ```

2. **Set up environment variables**
   ```bash
   cp .env.example .env
   ```
   Edit `.env` and set `POSTGRES_USER`, `POSTGRES_PASSWORD`, and `POSTGRES_DB` to the values you want. Keep `DATABASE_URL` consistent with those values.

3. **Start the database**
   ```bash
   docker compose up -d
   ```

4. **Run the application**
   ```bash
   cargo run
   ```
   On the first run, the application automatically applies any pending migrations (via `sqlx::migrate!`) before opening the menu — no manual step required.

## Project structure

```
src/
├── lib.rs          # Library struct, queries, error and domain types
├── main.rs         # main CLI loop
├── menu.rs         # menu display and option reading
├── add_book.rs     # book registration flow
├── book_list.rs    # listing flow
├── borrow_book.rs  # borrowing flow
└── return_book.rs  # return flow
migrations/         # versioned database schema (applied via sqlx)
docker-compose.yml  # local PostgreSQL service
```

## Design decisions

- **No in-memory cache**: every operation reads/writes directly to the database. This simplifies the model (single source of truth), at the cost of one extra query per operation — negligible at this project's scale.
- **`id` as `SERIAL`**: Postgres generates and guarantees ID uniqueness, completely removing the need to compute indexes manually.
- **Atomic conditional updates**: `borrow_book`/`return_book` use a single `UPDATE ... WHERE ... RETURNING` statement instead of a "check, then update" two-step approach — this avoids a race condition where two concurrent processes could borrow the same book at the same time.
- **Typed errors**: a custom `Error` enum wraps both user input failures and database failures (`sqlx::Error`, converted via `impl From`), keeping a single error surface across the application.
