CREATE TYPE referrer AS ENUM ('search', 'peer', 'social-media', 'blog', 'advertisement', 'school', 'student-organization', 'other');

ALTER TABLE applications
    ADD COLUMN referrer referrer DEFAULT NULL;
ALTER TABLE draft_applications
    ADD COLUMN referrer referrer DEFAULT NULL;
