-- Your SQL goes here
CREATE TABLE exercises (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  description TEXT,
  is_active BOOLEAN NOT NULL DEFAULT true,
  thumbnail_url TEXT,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE exercise_body_parts (
  id SERIAL PRIMARY KEY,
  exercise_id INTEGER NOT NULL REFERENCES exercises(id) ON DELETE CASCADE,
  body_part_id INTEGER NOT NULL REFERENCES body_parts(id) ON DELETE CASCADE,
  UNIQUE(exercise_id, body_part_id)
);

CREATE TABLE exercise_categories (
  id SERIAL PRIMARY KEY,
  exercise_id INTEGER NOT NULL REFERENCES exercises(id) ON DELETE CASCADE,
  category_id INTEGER NOT NULL REFERENCES categories(id) ON DELETE CASCADE,
  UNIQUE(exercise_id, category_id)
);

CREATE TABLE exercise_equipment (
  id SERIAL PRIMARY KEY,
  exercise_id INTEGER NOT NULL REFERENCES exercises(id) ON DELETE CASCADE,
  equipment_id INTEGER NOT NULL REFERENCES equipment(id) ON DELETE CASCADE,
  UNIQUE(exercise_id, equipment_id)
);