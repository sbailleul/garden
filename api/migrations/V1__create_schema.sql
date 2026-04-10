CREATE TABLE vegetables (
    id               TEXT    PRIMARY KEY,
    latin_name       TEXT    NOT NULL,
    category         TEXT    NOT NULL,
    lifecycle        TEXT    NOT NULL,
    spacing_cm       INTEGER NOT NULL,
    days_to_harvest  INTEGER NOT NULL,
    days_to_plant    INTEGER NOT NULL,
    beginner_friendly BOOLEAN NOT NULL,
    soil_types       TEXT[]  NOT NULL,
    sun_requirement  TEXT[]  NOT NULL,
    good_companions  TEXT[]  NOT NULL,
    bad_companions   TEXT[]  NOT NULL,
    calendars        JSONB   NOT NULL
);

CREATE TABLE vegetable_translations (
    vegetable_id TEXT NOT NULL REFERENCES vegetables(id) ON DELETE CASCADE,
    locale       TEXT NOT NULL,
    name         TEXT NOT NULL,
    PRIMARY KEY (vegetable_id, locale)
);
