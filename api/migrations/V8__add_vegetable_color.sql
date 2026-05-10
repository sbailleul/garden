-- ---------------------------------------------------------------------------
-- V8: Add representative color code to each vegetable
--
-- Each vegetable gets a unique 8-digit HEX color (RRGGBBAA, fully opaque)
-- that is visually representative of the plant.
-- ---------------------------------------------------------------------------

ALTER TABLE vegetables ADD COLUMN color TEXT NOT NULL DEFAULT '#808080FF';

UPDATE vegetables SET color = '#708238FF' WHERE id = 'artichoke';
UPDATE vegetables SET color = '#87A96BFF' WHERE id = 'asparagus';
UPDATE vegetables SET color = '#1A5C38FF' WHERE id = 'basil';
UPDATE vegetables SET color = '#8B0000FF' WHERE id = 'beet';
UPDATE vegetables SET color = '#556B2FFF' WHERE id = 'brassica';
UPDATE vegetables SET color = '#ED7014FF' WHERE id = 'carrot';
UPDATE vegetables SET color = '#ACE1AFFF' WHERE id = 'celery';
UPDATE vegetables SET color = '#4F7942FF' WHERE id = 'chive';
UPDATE vegetables SET color = '#2E8B57FF' WHERE id = 'cucumber';
UPDATE vegetables SET color = '#380835FF' WHERE id = 'eggplant';
UPDATE vegetables SET color = '#BFAF1EFF' WHERE id = 'fennel';
UPDATE vegetables SET color = '#EDE8D0FF' WHERE id = 'garlic';
UPDATE vegetables SET color = '#6B8E23FF' WHERE id = 'green-bean';
UPDATE vegetables SET color = '#3A5C3AFF' WHERE id = 'leek';
UPDATE vegetables SET color = '#8DB600FF' WHERE id = 'lettuce';
UPDATE vegetables SET color = '#FBCB23FF' WHERE id = 'maïs';
UPDATE vegetables SET color = '#3EB489FF' WHERE id = 'mint';
UPDATE vegetables SET color = '#C68642FF' WHERE id = 'onion';
UPDATE vegetables SET color = '#299617FF' WHERE id = 'parsley';
UPDATE vegetables SET color = '#93C572FF' WHERE id = 'pea';
UPDATE vegetables SET color = '#C0392BFF' WHERE id = 'pepper';
UPDATE vegetables SET color = '#9C8456FF' WHERE id = 'potato';
UPDATE vegetables SET color = '#FF7518FF' WHERE id = 'pumpkin';
UPDATE vegetables SET color = '#E75480FF' WHERE id = 'radish';
UPDATE vegetables SET color = '#4C516DFF' WHERE id = 'rosemary';
UPDATE vegetables SET color = '#006400FF' WHERE id = 'spinach';
UPDATE vegetables SET color = '#FC5A8DFF' WHERE id = 'strawberry';
UPDATE vegetables SET color = '#9375A0FF' WHERE id = 'thyme';
UPDATE vegetables SET color = '#E34234FF' WHERE id = 'tomato';
UPDATE vegetables SET color = '#C8A2C8FF' WHERE id = 'turnip';
UPDATE vegetables SET color = '#5D7B48FF' WHERE id = 'zucchini';

ALTER TABLE vegetables ALTER COLUMN color DROP DEFAULT;
