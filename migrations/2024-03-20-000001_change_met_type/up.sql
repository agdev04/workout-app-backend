-- Change MET column type from DECIMAL to INTEGER NOT NULL
-- First, update null values to a default value (0)
UPDATE exercises SET met = 0 WHERE met IS NULL;

-- Then change the column type to INTEGER
ALTER TABLE exercises
    ALTER COLUMN met TYPE INTEGER
    USING (ROUND(COALESCE(met, 0)))::INTEGER;

-- Finally, set the NOT NULL constraint
ALTER TABLE exercises
    ALTER COLUMN met SET NOT NULL;