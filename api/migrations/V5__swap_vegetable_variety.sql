-- ---------------------------------------------------------------------------
-- Swap table names: vegetables ↔ varieties
--
-- Before: vegetables = detailed plant data (tomato, basil …)
--         varieties  = grouping entities (brassica, tomato-group …)
-- After:  varieties  = detailed plant data
--         vegetables = grouping entities
-- ---------------------------------------------------------------------------

-- Step 1: Move grouping tables aside
ALTER TABLE varieties             RENAME TO __tmp_old_varieties__;
ALTER TABLE variety_translations  RENAME TO __tmp_old_variety_translations__;

-- Step 2: Rename detailed-plant tables to new names
ALTER TABLE vegetables            RENAME TO varieties;
ALTER TABLE vegetable_translations RENAME TO variety_translations;

-- Step 3: Promote grouping tables to the vegetable namespace
ALTER TABLE __tmp_old_varieties__            RENAME TO vegetables;
ALTER TABLE __tmp_old_variety_translations__ RENAME TO vegetable_translations;

-- Step 4: Rename FK columns to match their new owning tables
--   varieties.variety_id  (FK → vegetables.id)  → varieties.vegetable_id
ALTER TABLE varieties             RENAME COLUMN variety_id   TO vegetable_id;

--   vegetable_translations.variety_id (FK → vegetables.id) → vegetable_id
ALTER TABLE vegetable_translations RENAME COLUMN variety_id  TO vegetable_id;

--   variety_translations.vegetable_id (FK → varieties.id) → variety_id
ALTER TABLE variety_translations  RENAME COLUMN vegetable_id TO variety_id;
