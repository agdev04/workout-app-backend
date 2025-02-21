-- Create workout_progress table
CREATE TABLE workout_progress (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    workout_id INTEGER NOT NULL REFERENCES workouts(id) ON DELETE CASCADE,
    exercise_id INTEGER NOT NULL REFERENCES exercises(id) ON DELETE CASCADE,
    workout_exercise_id INTEGER NOT NULL REFERENCES workout_exercises(id) ON DELETE CASCADE,
    completed BOOLEAN NOT NULL DEFAULT false,
    actual_reps INTEGER,
    actual_sets_number INTEGER,
    actual_duration_seconds INTEGER,
    notes TEXT,
    completed_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CHECK (actual_reps IS NOT NULL OR actual_duration_seconds IS NOT NULL)
);

-- Create indexes for better query performance
CREATE INDEX workout_progress_user_id_idx ON workout_progress(user_id);
CREATE INDEX workout_progress_workout_id_idx ON workout_progress(workout_id);
CREATE INDEX workout_progress_exercise_id_idx ON workout_progress(exercise_id);
CREATE INDEX workout_progress_workout_exercise_id_idx ON workout_progress(workout_exercise_id);