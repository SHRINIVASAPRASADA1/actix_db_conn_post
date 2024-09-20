-- Your SQL goes here
-- Your SQL goes here

CREATE TABLE category (
    id  SERIAL PRIMARY KEY,
    title  VARCHAR NOT NULL,
    subtitle  VARCHAR NOT NULL,
    keywords  JSONB
);

CREATE TABLE products (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    category_id INTEGER NOT NULL REFERENCES category(id) ON DELETE CASCADE,
    price INT NOT NULL,
    description TEXT,
    discount INT,
    quantity INT,
    category VARCHAR,
    brand VARCHAR,
    thumbnail VARCHAR,
    weight REAL,
    height REAL,
    is_active BOOLEAN DEFAULT FALSE,
    tag JSONB,
    images JSONB,
    features JSONB
);
