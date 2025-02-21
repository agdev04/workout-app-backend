-- Your SQL goes here
CREATE TABLE workouts (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    description TEXT,
    difficulty VARCHAR NOT NULL,
    thumbnail_url TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE workout_exercises (
    id SERIAL PRIMARY KEY,
    workout_id INTEGER NOT NULL REFERENCES workouts(id) ON DELETE CASCADE,
    exercise_id INTEGER NOT NULL REFERENCES exercises(id) ON DELETE CASCADE,
    position INTEGER NOT NULL,
    sets_number INTEGER NOT NULL,
    reps INTEGER,
    duration_seconds INTEGER,
    rest_seconds INTEGER,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CHECK (reps IS NOT NULL OR duration_seconds IS NOT NULL)
);

CREATE INDEX workout_exercises_workout_id_idx ON workout_exercises(workout_id);
CREATE INDEX workout_exercises_exercise_id_idx ON workout_exercises(exercise_id);
