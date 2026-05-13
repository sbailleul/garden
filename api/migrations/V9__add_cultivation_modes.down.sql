-- ---------------------------------------------------------------------------
-- Rollback V9: remove height strata + cultivation modes
-- ---------------------------------------------------------------------------

-- Restore spacing_cm on varieties
ALTER TABLE varieties ADD COLUMN spacing_cm INTEGER NOT NULL DEFAULT 0;

-- Restore values from cultivation_modes before dropping them
UPDATE varieties v
SET spacing_cm = cm.spacing_cm
FROM cultivation_modes cm
WHERE cm.variety_id = v.id
  AND cm.id = v.id || '-standard';

-- Drop tables in reverse dependency order
DROP TABLE IF EXISTS cultivation_mode_translations;
DROP TABLE IF EXISTS cultivation_modes;
DROP TABLE IF EXISTS stratum_translations;
DROP TABLE IF EXISTS strata;
