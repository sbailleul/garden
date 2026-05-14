-- ---------------------------------------------------------------------------
-- Migration V10: add min/max plant height to cultivation_modes
-- ---------------------------------------------------------------------------

ALTER TABLE cultivation_modes ADD COLUMN min_height_cm INTEGER NOT NULL DEFAULT 0;
ALTER TABLE cultivation_modes ADD COLUMN max_height_cm INTEGER NOT NULL DEFAULT 0;

-- Seed representative heights per stratum
UPDATE cultivation_modes SET min_height_cm = 10,  max_height_cm = 40  WHERE stratum_id = 'ground-cover';
UPDATE cultivation_modes SET min_height_cm = 40,  max_height_cm = 120 WHERE stratum_id = 'intermediate';
UPDATE cultivation_modes SET min_height_cm = 100, max_height_cm = 220 WHERE stratum_id = 'canopy';
