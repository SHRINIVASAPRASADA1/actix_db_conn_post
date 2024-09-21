-- Create user_address table
CREATE TABLE user_address (
    id SERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
    pin_code INTEGER,
    area VARCHAR,
    city VARCHAR,
    contact VARCHAR,
    added_date DATE DEFAULT CURRENT_DATE,
    other VARCHAR
);

-- Create user_cart table
CREATE TABLE user_cart (
    id SERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
    product_id INTEGER,
    quantity INTEGER,
    added_date DATE DEFAULT CURRENT_DATE,
    is_active BOOLEAN DEFAULT FALSE,
    extra JSONB
);

-- Create user_order table
CREATE TABLE user_order (
    id SERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
    product_id INTEGER REFERENCES products(id) ON DELETE CASCADE,
    quantity INTEGER,
    added_date DATE DEFAULT CURRENT_DATE,
    is_accepted BOOLEAN DEFAULT FALSE,
    is_cancelled BOOLEAN DEFAULT FALSE,
    is_online_pay BOOLEAN DEFAULT TRUE,
    extra JSONB
);

-- Create order_status table
CREATE TABLE order_status (
    id SERIAL PRIMARY KEY,
    product_id INTEGER REFERENCES products(id) ON DELETE CASCADE,
    order_id INTEGER REFERENCES user_order(id) ON DELETE CASCADE,
    last_updated DATE DEFAULT CURRENT_DATE,
    is_accepted BOOLEAN DEFAULT FALSE,
    locations JSONB
);
