-- Your SQL goes here
CREATE TABLE products (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    price INT NOT NULL,
    description TEXT,
    discount INT,
    quantity INT,
    category VARCHAR,
    brand VARCHAR,
    thumbnail VARCHAR,
    weight REAL,
    height REAL,
    is_active BOOLEAN DEFAULT TRUE,
    tag JSONB,
    images JSONB,
    features JSONB
);
