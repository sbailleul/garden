-- ---------------------------------------------------------------------------
-- V8 rollback: Remove the color column from vegetables
-- ---------------------------------------------------------------------------

ALTER TABLE vegetables DROP COLUMN IF EXISTS color;
