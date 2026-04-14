-- ---------------------------------------------------------------------------
-- Schema: varieties
-- ---------------------------------------------------------------------------

CREATE TABLE varieties (
    id TEXT PRIMARY KEY
);

CREATE TABLE variety_translations (
    variety_id TEXT NOT NULL REFERENCES varieties(id) ON DELETE CASCADE,
    locale     TEXT NOT NULL,
    name       TEXT NOT NULL,
    PRIMARY KEY (variety_id, locale)
);

ALTER TABLE vegetables
    ADD COLUMN variety_id TEXT REFERENCES varieties(id);

-- ---------------------------------------------------------------------------
-- Seed: varieties
-- ---------------------------------------------------------------------------

INSERT INTO varieties (id) VALUES
('tomato'),
('pepper'),
('zucchini'),
('pumpkin'),
('cucumber'),
('eggplant'),
('carrot'),
('beet'),
('radish'),
('turnip'),
('potato'),
('onion'),
('garlic'),
('leek'),
('chive'),
('green-bean'),
('pea'),
('maïs'),
('brassica'),
('lettuce'),
('spinach'),
('celery'),
('asparagus'),
('artichoke'),
('basil'),
('parsley'),
('thyme'),
('rosemary'),
('fennel'),
('mint'),
('strawberry');

INSERT INTO variety_translations (variety_id, locale, name) VALUES
('tomato',      'en', 'Tomato'),
('tomato',      'fr', 'Tomate'),
('pepper',      'en', 'Pepper'),
('pepper',      'fr', 'Poivron'),
('zucchini',    'en', 'Zucchini'),
('zucchini',    'fr', 'Courgette'),
('pumpkin',     'en', 'Pumpkin'),
('pumpkin',     'fr', 'Citrouille'),
('cucumber',    'en', 'Cucumber'),
('cucumber',    'fr', 'Concombre'),
('eggplant',    'en', 'Eggplant'),
('eggplant',    'fr', 'Aubergine'),
('carrot',      'en', 'Carrot'),
('carrot',      'fr', 'Carotte'),
('beet',        'en', 'Beet'),
('beet',        'fr', 'Betterave'),
('radish',      'en', 'Radish'),
('radish',      'fr', 'Radis'),
('turnip',      'en', 'Turnip'),
('turnip',      'fr', 'Navet'),
('potato',      'en', 'Potato'),
('potato',      'fr', 'Pomme de terre'),
('onion',       'en', 'Onion'),
('onion',       'fr', 'Oignon'),
('garlic',      'en', 'Garlic'),
('garlic',      'fr', 'Ail'),
('leek',        'en', 'Leek'),
('leek',        'fr', 'Poireau'),
('chive',       'en', 'Chive'),
('chive',       'fr', 'Ciboulette'),
('green-bean',  'en', 'Green Bean'),
('green-bean',  'fr', 'Haricot vert'),
('pea',         'en', 'Pea'),
('pea',         'fr', 'Pois'),
('maïs',        'en', 'Corn'),
('maïs',        'fr', 'Maïs'),
('brassica',    'en', 'Brassica'),
('brassica',    'fr', 'Brassica'),
('lettuce',     'en', 'Lettuce'),
('lettuce',     'fr', 'Laitue'),
('spinach',     'en', 'Spinach'),
('spinach',     'fr', 'Épinard'),
('celery',      'en', 'Celery'),
('celery',      'fr', 'Céleri'),
('asparagus',   'en', 'Asparagus'),
('asparagus',   'fr', 'Asperge'),
('artichoke',   'en', 'Artichoke'),
('artichoke',   'fr', 'Artichaut'),
('basil',       'en', 'Basil'),
('basil',       'fr', 'Basilic'),
('parsley',     'en', 'Parsley'),
('parsley',     'fr', 'Persil'),
('thyme',       'en', 'Thyme'),
('thyme',       'fr', 'Thym'),
('rosemary',    'en', 'Rosemary'),
('rosemary',    'fr', 'Romarin'),
('fennel',      'en', 'Fennel'),
('fennel',      'fr', 'Fenouil'),
('mint',        'en', 'Mint'),
('mint',        'fr', 'Menthe'),
('strawberry',  'en', 'Strawberry'),
('strawberry',  'fr', 'Fraise');

-- ---------------------------------------------------------------------------
-- Link vegetables to their varieties
-- ---------------------------------------------------------------------------

UPDATE vegetables SET variety_id = 'tomato'     WHERE id = 'tomato';
UPDATE vegetables SET variety_id = 'pepper'     WHERE id IN ('pepper', 'red-pepper');
UPDATE vegetables SET variety_id = 'zucchini'   WHERE id = 'zucchini';
UPDATE vegetables SET variety_id = 'pumpkin'    WHERE id = 'pumpkin';
UPDATE vegetables SET variety_id = 'cucumber'   WHERE id = 'cucumber';
UPDATE vegetables SET variety_id = 'eggplant'   WHERE id = 'eggplant';
UPDATE vegetables SET variety_id = 'carrot'     WHERE id = 'carrot';
UPDATE vegetables SET variety_id = 'beet'       WHERE id = 'beet';
UPDATE vegetables SET variety_id = 'radish'     WHERE id = 'radish';
UPDATE vegetables SET variety_id = 'turnip'     WHERE id = 'turnip';
UPDATE vegetables SET variety_id = 'potato'     WHERE id = 'potato';
UPDATE vegetables SET variety_id = 'onion'      WHERE id = 'onion';
UPDATE vegetables SET variety_id = 'garlic'     WHERE id = 'garlic';
UPDATE vegetables SET variety_id = 'leek'       WHERE id = 'leek';
UPDATE vegetables SET variety_id = 'chive'      WHERE id = 'chive';
UPDATE vegetables SET variety_id = 'green-bean' WHERE id = 'green-bean';
UPDATE vegetables SET variety_id = 'pea'        WHERE id = 'pea';
UPDATE vegetables SET variety_id = 'maïs'       WHERE id = 'maïs';
UPDATE vegetables SET variety_id = 'brassica'   WHERE id IN ('cabbage', 'broccoli', 'cauliflower');
UPDATE vegetables SET variety_id = 'lettuce'    WHERE id = 'lettuce';
UPDATE vegetables SET variety_id = 'spinach'    WHERE id = 'spinach';
UPDATE vegetables SET variety_id = 'celery'     WHERE id = 'celery';
UPDATE vegetables SET variety_id = 'asparagus'  WHERE id = 'asparagus';
UPDATE vegetables SET variety_id = 'artichoke'  WHERE id = 'artichoke';
UPDATE vegetables SET variety_id = 'basil'      WHERE id = 'basil';
UPDATE vegetables SET variety_id = 'parsley'    WHERE id = 'parsley';
UPDATE vegetables SET variety_id = 'thyme'      WHERE id = 'thyme';
UPDATE vegetables SET variety_id = 'rosemary'   WHERE id = 'rosemary';
UPDATE vegetables SET variety_id = 'fennel'     WHERE id = 'fennel';
UPDATE vegetables SET variety_id = 'mint'       WHERE id = 'mint';
UPDATE vegetables SET variety_id = 'strawberry' WHERE id = 'strawberry';
