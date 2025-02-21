CREATE TABLE programme_progress (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id),
    programme_id INTEGER NOT NULL REFERENCES programmes(id),
    programme_week_id INTEGER NOT NULL REFERENCES programme_weeks(id),
    exercise_id INTEGER NOT NULL REFERENCES exercises(id),
    day_number INTEGER NOT NULL,
    completed BOOLEAN NOT NULL DEFAULT false,
    actual_reps INTEGER,
    actual_duration_seconds INTEGER,
    actual_rest_seconds INTEGER,
    notes TEXT,
    completed_at TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT unique_user_programme_exercise UNIQUE (user_id, programme_id, programme_week_id, exercise_id, day_number)
);

SELECT diesel_manage_updated_at('programme_progress');