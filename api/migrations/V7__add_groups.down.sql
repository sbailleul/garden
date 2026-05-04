-- ---------------------------------------------------------------------------
-- V7 rollback: Remove groups
-- ---------------------------------------------------------------------------

ALTER TABLE vegetables DROP COLUMN IF EXISTS group_id;

DROP TABLE IF EXISTS group_translations;
DROP TABLE IF EXISTS groups;
