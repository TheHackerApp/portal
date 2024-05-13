CREATE TABLE draft_applications
(
    event               text                     not null,
    participant_id      int                      not null,

    gender              gender,
    race_ethnicity      race_ethnicity,
    date_of_birth       date,

    education           education,
    graduation_year     int,
    major               text,

    hackathons_attended int,
    links               text[],

    address_line1       text,
    address_line2       text,
    address_line3       text,
    locality            text,
    administrative_area text,
    postal_code         text,
    country             text,

    share_information   bool,

    created_at          timestamp with time zone not null default now(),
    updated_at          timestamp with time zone not null default now(),

    primary key (participant_id, event)
);

CREATE TRIGGER set_draft_applications_updated_at_timestamp
    BEFORE UPDATE
    ON draft_applications
    FOR EACH ROW
EXECUTE PROCEDURE set_updated_at_timestamp();
