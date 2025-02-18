CREATE TABLE programmes (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    description TEXT,
    thumbnail_url TEXT,
    weeks INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE programme_weeks (
    id SERIAL PRIMARY KEY,
    programme_id INTEGER NOT NULL REFERENCES programmes(id) ON DELETE CASCADE,
    week_number INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(programme_id, week_number)
);

CREATE TABLE programme_days (
    id SERIAL PRIMARY KEY,
    programme_week_id INTEGER NOT NULL REFERENCES programme_weeks(id) ON DELETE CASCADE,
    day_number INTEGER NOT NULL CHECK (day_number BETWEEN 1 AND 7),
    exercise_id INTEGER NOT NULL REFERENCES exercises(id),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);