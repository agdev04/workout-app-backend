-- Your SQL goes here
CREATE TABLE programmes (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    description TEXT,
    image_url TEXT,
    total_weeks INTEGER NOT NULL CHECK (total_weeks > 0),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE programme_weeks (
    id SERIAL PRIMARY KEY,
    programme_id INTEGER NOT NULL REFERENCES programmes(id) ON DELETE CASCADE,
    name VARCHAR NOT NULL,
    week_number INTEGER NOT NULL CHECK (week_number > 0),
    UNIQUE(programme_id, week_number)
);

CREATE TABLE programme_days_exercises (
    id SERIAL PRIMARY KEY,
    programme_week_id INTEGER NOT NULL REFERENCES programme_weeks(id) ON DELETE CASCADE,
    exercise_id INTEGER NOT NULL REFERENCES exercises(id) ON DELETE CASCADE,
    day_number INTEGER NOT NULL CHECK (day_number BETWEEN 1 AND 7),
    position INTEGER NOT NULL,
    reps INTEGER,
    duration_seconds INTEGER,
    rest_seconds INTEGER NOT NULL,
    CHECK (reps IS NOT NULL OR duration_seconds IS NOT NULL),
    UNIQUE(programme_week_id, day_number, position)
);

CREATE INDEX programme_weeks_programme_id_idx ON programme_weeks(programme_id);
CREATE INDEX programme_days_exercises_week_id_idx ON programme_days_exercises(programme_week_id);
CREATE INDEX programme_days_exercises_exercise_id_idx ON programme_days_exercises(exercise_id);

SELECT diesel_manage_updated_at('programmes');
SELECT diesel_manage_updated_at('programme_weeks');
SELECT diesel_manage_updated_at('programme_days_exercises');