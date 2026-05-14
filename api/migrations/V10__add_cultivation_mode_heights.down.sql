-- ---------------------------------------------------------------------------
-- Rollback V10: remove min/max plant height from cultivation_modes
-- ---------------------------------------------------------------------------

ALTER TABLE cultivation_modes DROP COLUMN IF EXISTS max_height_cm;
ALTER TABLE cultivation_modes DROP COLUMN IF EXISTS min_height_cm;
