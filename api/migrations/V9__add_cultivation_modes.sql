-- ---------------------------------------------------------------------------
-- Migration V9: height strata + cultivation modes
-- ---------------------------------------------------------------------------
-- Adds:
--   strata                       - three height layers
--   stratum_translations         - i18n names
--   cultivation_modes            - how a variety is grown (spacing, stratum)
--   cultivation_mode_translations
-- Seeds one "standard" cultivation mode per variety.
-- Drops varieties.spacing_cm (moved into cultivation_modes).
-- ---------------------------------------------------------------------------

CREATE TABLE strata (
    id TEXT PRIMARY KEY
);

CREATE TABLE stratum_translations (
    stratum_id TEXT NOT NULL REFERENCES strata(id) ON DELETE CASCADE,
    locale     TEXT NOT NULL,
    name       TEXT NOT NULL,
    PRIMARY KEY (stratum_id, locale)
);

CREATE TABLE cultivation_modes (
    id         TEXT    PRIMARY KEY,
    variety_id TEXT    NOT NULL REFERENCES varieties(id) ON DELETE CASCADE,
    stratum_id TEXT    NOT NULL REFERENCES strata(id),
    spacing_cm INTEGER NOT NULL
);

CREATE TABLE cultivation_mode_translations (
    cultivation_mode_id TEXT NOT NULL REFERENCES cultivation_modes(id) ON DELETE CASCADE,
    locale              TEXT NOT NULL,
    name                TEXT NOT NULL,
    PRIMARY KEY (cultivation_mode_id, locale)
);

-- ---------------------------------------------------------------------------
-- Seed: strata
-- ---------------------------------------------------------------------------

INSERT INTO strata (id) VALUES
    ('ground-cover'),
    ('intermediate'),
    ('canopy');

INSERT INTO stratum_translations (stratum_id, locale, name) VALUES
    ('ground-cover', 'en', 'Ground Cover'),
    ('ground-cover', 'fr', 'Plante couvre-sol'),
    ('intermediate', 'en', 'Intermediate'),
    ('intermediate', 'fr', 'Intermédiaire'),
    ('canopy',       'en', 'Canopy'),
    ('canopy',       'fr', 'Grande taille');

-- ---------------------------------------------------------------------------
-- Seed: cultivation modes (one per variety, reading spacing_cm from current data)
-- Ground cover  : radish, carrot, onion, garlic, chive, basil, parsley,
--                 beet, spinach, lettuce, turnip, strawberry, thyme, mint
-- Canopy        : zucchini, cucumber, maïs, pumpkin, asparagus, artichoke, fennel
-- Intermediate  : everything else
-- ---------------------------------------------------------------------------

INSERT INTO cultivation_modes (id, variety_id, stratum_id, spacing_cm)
SELECT
    v.id || '-standard',
    v.id,
    CASE v.id
        WHEN 'radish'     THEN 'ground-cover'
        WHEN 'carrot'     THEN 'ground-cover'
        WHEN 'onion'      THEN 'ground-cover'
        WHEN 'garlic'     THEN 'ground-cover'
        WHEN 'chive'      THEN 'ground-cover'
        WHEN 'basil'      THEN 'ground-cover'
        WHEN 'parsley'    THEN 'ground-cover'
        WHEN 'beet'       THEN 'ground-cover'
        WHEN 'spinach'    THEN 'ground-cover'
        WHEN 'lettuce'    THEN 'ground-cover'
        WHEN 'turnip'     THEN 'ground-cover'
        WHEN 'strawberry' THEN 'ground-cover'
        WHEN 'thyme'      THEN 'ground-cover'
        WHEN 'mint'       THEN 'ground-cover'
        WHEN 'zucchini'   THEN 'canopy'
        WHEN 'cucumber'   THEN 'canopy'
        WHEN 'maïs'       THEN 'canopy'
        WHEN 'pumpkin'    THEN 'canopy'
        WHEN 'asparagus'  THEN 'canopy'
        WHEN 'artichoke'  THEN 'canopy'
        WHEN 'fennel'     THEN 'canopy'
        ELSE 'intermediate'
    END,
    v.spacing_cm
FROM varieties v;

-- ---------------------------------------------------------------------------
-- Seed: cultivation mode translations (en + fr — "Standard" for both locales)
-- ---------------------------------------------------------------------------

INSERT INTO cultivation_mode_translations (cultivation_mode_id, locale, name)
SELECT cm.id, 'en', 'Standard'
FROM cultivation_modes cm;

INSERT INTO cultivation_mode_translations (cultivation_mode_id, locale, name)
SELECT cm.id, 'fr', 'Standard'
FROM cultivation_modes cm;

-- ---------------------------------------------------------------------------
-- Remove spacing_cm from varieties (now lives in cultivation_modes)
-- ---------------------------------------------------------------------------

ALTER TABLE varieties DROP COLUMN spacing_cm;
