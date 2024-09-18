-- Your SQL goes here
CREATE TABLE products (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    price INT NOT NULL,
    description TEXT NOT NULL
);
