CREATE TABLE meals (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    category VARCHAR NOT NULL,
    description TEXT,
    image_url TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE meal_ingredients (
    id SERIAL PRIMARY KEY,
    meal_id INTEGER NOT NULL REFERENCES meals(id) ON DELETE CASCADE,
    name VARCHAR NOT NULL,
    amount VARCHAR NOT NULL
);

CREATE TABLE meal_instructions (
    id SERIAL PRIMARY KEY,
    meal_id INTEGER NOT NULL REFERENCES meals(id) ON DELETE CASCADE,
    step_number INTEGER NOT NULL,
    instruction TEXT NOT NULL
);