ALTER TABLE draft_applications
    ALTER COLUMN share_information DROP NOT NULL,
    ALTER COLUMN share_information DROP DEFAULT;

ALTER TABLE draft_applications
    ALTER COLUMN links DROP NOT NULL,
    ALTER COLUMN links DROP DEFAULT;