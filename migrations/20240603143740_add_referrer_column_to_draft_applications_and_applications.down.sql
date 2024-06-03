ALTER TABLE applications
    DROP COLUMN referrer;
ALTER TABLE draft_applications
    DROP COLUMN referrer;

DROP TYPE referrer;
