-- ---------------------------------------------------------------------------
-- V7: Add groups table
-- A group is a high-level botanical/culinary category that contains several
-- vegetables (e.g. "Bulbes" contains onion, garlic, leek, chive).
-- ---------------------------------------------------------------------------

CREATE TABLE groups (
    id TEXT PRIMARY KEY
);

CREATE TABLE group_translations (
    group_id TEXT NOT NULL REFERENCES groups(id),
    locale   TEXT NOT NULL,
    name     TEXT NOT NULL,
    PRIMARY KEY (group_id, locale)
);

-- ---------------------------------------------------------------------------
-- Seed: groups
-- ---------------------------------------------------------------------------

INSERT INTO groups (id) VALUES
    ('bulbes'),
    ('engrais-verts'),
    ('legumes-feuilles'),
    ('legumes-fruits'),
    ('legumes-racines'),
    ('plantes-a-grains');

INSERT INTO group_translations (group_id, locale, name) VALUES
    ('bulbes',           'fr', 'Bulbes'),
    ('bulbes',           'en', 'Bulbs'),
    ('engrais-verts',    'fr', 'Engrais Verts'),
    ('engrais-verts',    'en', 'Green Manure'),
    ('legumes-feuilles', 'fr', 'Légumes-Feuilles'),
    ('legumes-feuilles', 'en', 'Leafy Vegetables'),
    ('legumes-fruits',   'fr', 'Légumes-Fruits'),
    ('legumes-fruits',   'en', 'Fruiting Vegetables'),
    ('legumes-racines',  'fr', 'Légumes-Racines'),
    ('legumes-racines',  'en', 'Root Vegetables'),
    ('plantes-a-grains', 'fr', 'Plantes à Grains'),
    ('plantes-a-grains', 'en', 'Grain Plants');

-- ---------------------------------------------------------------------------
-- Link vegetables to their groups
-- Add nullable first, seed all rows, then enforce NOT NULL
-- ---------------------------------------------------------------------------

ALTER TABLE vegetables ADD COLUMN group_id TEXT REFERENCES groups(id);

UPDATE vegetables SET group_id = 'bulbes'
WHERE id IN ('onion', 'garlic', 'leek', 'chive');

UPDATE vegetables SET group_id = 'legumes-racines'
WHERE id IN ('carrot', 'radish', 'beet', 'potato', 'turnip');

UPDATE vegetables SET group_id = 'legumes-fruits'
WHERE id IN ('tomato', 'zucchini', 'cucumber', 'pepper', 'red-pepper',
             'eggplant', 'pumpkin', 'strawberry', 'green-bean', 'pea', 'maïs');

UPDATE vegetables SET group_id = 'legumes-feuilles'
WHERE id IN ('lettuce', 'cabbage', 'broccoli', 'brassica', 'spinach', 'celery',
             'cauliflower', 'asparagus', 'artichoke', 'basil', 'parsley',
             'thyme', 'rosemary', 'fennel', 'mint');

ALTER TABLE vegetables ALTER COLUMN group_id SET NOT NULL;
