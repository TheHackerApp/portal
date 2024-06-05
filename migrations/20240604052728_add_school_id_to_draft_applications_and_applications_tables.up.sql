ALTER TABLE draft_applications
    ADD COLUMN school_id uuid default null references schools (id);

ALTER TABLE applications
    ADD COLUMN school_id uuid references schools (id);
