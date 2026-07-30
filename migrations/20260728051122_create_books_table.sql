CREATE TABLE books (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    author TEXT NOT NULL,
    borrowable BOOL NOT NULL DEFAULT true
);
