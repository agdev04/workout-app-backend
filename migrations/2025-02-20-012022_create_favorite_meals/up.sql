-- Your SQL goes here
CREATE TABLE favorite_meals (
  id SERIAL PRIMARY KEY,
  user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  meal_id INTEGER NOT NULL REFERENCES meals(id) ON DELETE CASCADE,
  UNIQUE(user_id, meal_id)
);