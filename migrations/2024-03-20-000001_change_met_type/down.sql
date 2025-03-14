-- Revert MET column type back to DECIMAL
ALTER TABLE exercises
    ALTER COLUMN met TYPE DECIMAL
    USING met::DECIMAL;

ALTER TABLE exercises
    ALTER COLUMN met SET NOT NULL;