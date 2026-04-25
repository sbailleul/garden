-- ---------------------------------------------------------------------------
-- Move good_companions / bad_companions from varieties → vegetables
-- ---------------------------------------------------------------------------

-- Step 1: Add companion columns to vegetables
ALTER TABLE vegetables
    ADD COLUMN good_companions TEXT[] NOT NULL DEFAULT '{}',
    ADD COLUMN bad_companions  TEXT[] NOT NULL DEFAULT '{}';

-- Step 2: Populate companion data for each vegetable (using vegetable IDs)
-- Notes:
--   • For vegetables where vegetable_id = variety_id (most cases), companion IDs
--     are already vegetable IDs.
--   • "brassica" aggregates cabbage, broccoli, cauliflower → cabbage's companions used.
--   • "pepper" aggregates pepper + red-pepper → pepper's companions used.
--   • Companion IDs that reference non-existent vegetables (e.g. rose, sage, nasturtium,
--     hyssop, dill, mustard, sorrel) are kept as-is; they simply won't resolve at runtime.
--   • "broccoli" / "cabbage" / "cauliflower" / "red-pepper" references in bad_companions
--     are normalised to their vegetable IDs ("brassica" / "pepper").

UPDATE vegetables SET good_companions = '{basil,carrot,parsley,garlic,onion}',    bad_companions  = '{fennel,brassica}'                               WHERE id = 'tomato';
UPDATE vegetables SET good_companions = '{green-bean,maïs,radish,nasturtium}',    bad_companions  = '{potato}'                                        WHERE id = 'zucchini';
UPDATE vegetables SET good_companions = '{tomato,onion,leek,lettuce,radish}',      bad_companions  = '{dill,fennel}'                                   WHERE id = 'carrot';
UPDATE vegetables SET good_companions = '{tomato,pepper,asparagus}',               bad_companions  = '{sage,thyme}'                                    WHERE id = 'basil';
UPDATE vegetables SET good_companions = '{carrot,radish,strawberry,cucumber}',     bad_companions  = '{parsley,celery}'                                WHERE id = 'lettuce';
UPDATE vegetables SET good_companions = '{carrot,lettuce,tomato,cucumber}',        bad_companions  = '{hyssop}'                                        WHERE id = 'radish';
UPDATE vegetables SET good_companions = '{carrot,tomato,beet,lettuce}',            bad_companions  = '{green-bean,pea,garlic}'                         WHERE id = 'onion';
UPDATE vegetables SET good_companions = '{tomato,rose,strawberry,carrot}',         bad_companions  = '{onion,green-bean,pea}'                          WHERE id = 'garlic';
UPDATE vegetables SET good_companions = '{carrot,celery,lettuce}',                 bad_companions  = '{green-bean,pea}'                                WHERE id = 'leek';
UPDATE vegetables SET good_companions = '{zucchini,maïs,potato,radish}',           bad_companions  = '{onion,garlic,fennel,leek}'                      WHERE id = 'green-bean';
UPDATE vegetables SET good_companions = '{radish,lettuce,green-bean,maïs}',        bad_companions  = '{tomato,potato,fennel}'                          WHERE id = 'cucumber';
UPDATE vegetables SET good_companions = '{basil,tomato,carrot}',                   bad_companions  = '{fennel,brassica}'                               WHERE id = 'pepper';
UPDATE vegetables SET good_companions = '{carrot,radish,lettuce,brassica}',        bad_companions  = '{onion,garlic,fennel}'                           WHERE id = 'pea';
UPDATE vegetables SET good_companions = '{celery,onion,pea}',                      bad_companions  = '{tomato,strawberry,fennel}'                      WHERE id = 'brassica';
UPDATE vegetables SET good_companions = '{tomato,asparagus,rose}',                 bad_companions  = '{lettuce}'                                       WHERE id = 'parsley';
UPDATE vegetables SET good_companions = '{brassica,tomato,eggplant}',              bad_companions  = '{basil}'                                         WHERE id = 'thyme';
UPDATE vegetables SET good_companions = '{brassica,green-bean,sage}',              bad_companions  = '{cucumber,pumpkin}'                              WHERE id = 'rosemary';
UPDATE vegetables SET good_companions = '{onion,lettuce,radish}',                  bad_companions  = '{green-bean,mustard}'                            WHERE id = 'beet';
UPDATE vegetables SET good_companions = '{strawberry,tomato,radish}',              bad_companions  = '{beet,sorrel}'                                   WHERE id = 'spinach';
UPDATE vegetables SET good_companions = '{}',                                      bad_companions  = '{tomato,green-bean,pepper,carrot,brassica,pea,cucumber}' WHERE id = 'fennel';
UPDATE vegetables SET good_companions = '{basil,thyme,pepper}',                    bad_companions  = '{fennel}'                                        WHERE id = 'eggplant';
UPDATE vegetables SET good_companions = '{leek,brassica,tomato}',                  bad_companions  = '{lettuce,garlic}'                                WHERE id = 'celery';
UPDATE vegetables SET good_companions = '{green-bean,brassica,maïs}',              bad_companions  = '{tomato,cucumber,zucchini}'                      WHERE id = 'potato';
UPDATE vegetables SET good_companions = '{green-bean,zucchini,potato}',            bad_companions  = '{tomato,celery}'                                 WHERE id = 'maïs';
UPDATE vegetables SET good_companions = '{maïs,green-bean,onion}',                 bad_companions  = '{potato,rosemary}'                               WHERE id = 'pumpkin';
UPDATE vegetables SET good_companions = '{carrot,tomato,rose,strawberry}',         bad_companions  = '{green-bean,pea}'                                WHERE id = 'chive';
UPDATE vegetables SET good_companions = '{brassica,tomato,pea}',                   bad_companions  = '{parsley}'                                       WHERE id = 'mint';
UPDATE vegetables SET good_companions = '{lettuce,spinach,garlic,onion}',          bad_companions  = '{brassica,fennel}'                               WHERE id = 'strawberry';
UPDATE vegetables SET good_companions = '{pea,green-bean}',                        bad_companions  = '{mustard,radish}'                                WHERE id = 'turnip';
UPDATE vegetables SET good_companions = '{tomato,parsley,basil}',                  bad_companions  = '{onion,garlic}'                                  WHERE id = 'asparagus';
UPDATE vegetables SET good_companions = '{brassica,lettuce}',                      bad_companions  = '{green-bean,tomato}'                             WHERE id = 'artichoke';

-- Step 3: Remove companion columns from varieties
ALTER TABLE varieties
    DROP COLUMN good_companions,
    DROP COLUMN bad_companions;
