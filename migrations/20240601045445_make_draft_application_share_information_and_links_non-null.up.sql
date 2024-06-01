UPDATE draft_applications
SET share_information = true
WHERE share_information IS NULL;

ALTER TABLE draft_applications
    ALTER COLUMN share_information SET DEFAULT true,
    ALTER COLUMN share_information SET NOT NULL;

UPDATE draft_applications
SET links = array []::text[]
WHERE links IS NULL;

ALTER TABLE draft_applications
    ALTER COLUMN links SET DEFAULT array []::text[],
    ALTER COLUMN links SET NOT NULL;
