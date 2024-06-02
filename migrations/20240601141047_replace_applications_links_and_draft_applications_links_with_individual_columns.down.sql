ALTER TABLE applications
    ADD COLUMN links text[] default array []::text[],
    DROP COLUMN vcs_url,
    DROP COLUMN portfolio_url,
    DROP COLUMN devpost_url;

ALTER TABLE draft_applications
    ADD COLUMN links text[] default array []::text[],
    DROP COLUMN vcs_url,
    DROP COLUMN portfolio_url,
    DROP COLUMN devpost_url;

