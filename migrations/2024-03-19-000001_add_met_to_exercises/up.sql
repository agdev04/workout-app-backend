-- Add MET (Metabolic Equivalent of Task) field to exercises table
ALTER TABLE exercises
ADD COLUMN met DECIMAL;