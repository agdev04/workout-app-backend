-- Your SQL goes here
CREATE TABLE favorite_workouts (
  id SERIAL PRIMARY KEY,
  user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  workout_id INTEGER NOT NULL REFERENCES workouts(id) ON DELETE CASCADE,
  UNIQUE(user_id, workout_id)
);