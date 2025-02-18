CREATE TABLE body_parts (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

SELECT diesel_manage_updated_at('body_parts');