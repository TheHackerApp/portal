ALTER TABLE applications
    DROP COLUMN links,
    ADD COLUMN vcs_url       text,
    ADD COLUMN portfolio_url text,
    ADD COLUMN devpost_url   text;

ALTER TABLE draft_applications
    DROP COLUMN links,
    ADD COLUMN vcs_url       text,
    ADD COLUMN portfolio_url text,
    ADD COLUMN devpost_url   text;
