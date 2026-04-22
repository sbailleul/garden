-- Reverse the column renames first
ALTER TABLE variety_translations  RENAME COLUMN variety_id   TO vegetable_id;
ALTER TABLE vegetable_translations RENAME COLUMN vegetable_id TO variety_id;
ALTER TABLE varieties             RENAME COLUMN vegetable_id  TO variety_id;

-- Reverse the table renames
ALTER TABLE vegetables            RENAME TO __tmp_old_vegetables__;
ALTER TABLE vegetable_translations RENAME TO __tmp_old_vegetable_translations__;

ALTER TABLE varieties             RENAME TO vegetables;
ALTER TABLE variety_translations  RENAME TO vegetable_translations;

ALTER TABLE __tmp_old_vegetables__            RENAME TO varieties;
ALTER TABLE __tmp_old_vegetable_translations__ RENAME TO variety_translations;
