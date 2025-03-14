-- This file should undo anything in `up.sql`
ALTER TABLE meals
DROP COLUMN difficulty,
DROP COLUMN carbs,
DROP COLUMN fat,
DROP COLUMN protein,
DROP COLUMN calories,
DROP COLUMN servings,
DROP COLUMN prep_time;