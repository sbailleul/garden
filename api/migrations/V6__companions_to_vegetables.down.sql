-- ---------------------------------------------------------------------------
-- Rollback: move good_companions / bad_companions back from vegetables → varieties
-- ---------------------------------------------------------------------------

-- Step 1: Re-add companion columns to varieties
ALTER TABLE varieties
    ADD COLUMN good_companions TEXT[] NOT NULL DEFAULT '{}',
    ADD COLUMN bad_companions  TEXT[] NOT NULL DEFAULT '{}';

-- Step 2: Restore companion data on varieties from the vegetable-level data.
-- For multi-variety vegetables (brassica, pepper), all varieties inherit the
-- vegetable's companion list.
UPDATE varieties vr
SET    good_companions = veg.good_companions,
       bad_companions  = veg.bad_companions
FROM   vegetables veg
WHERE  veg.id = vr.vegetable_id;

-- Step 3: Remove companion columns from vegetables
ALTER TABLE vegetables
    DROP COLUMN good_companions,
    DROP COLUMN bad_companions;
