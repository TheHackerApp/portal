CREATE TYPE gender AS ENUM ('male', 'female', 'non-binary', 'other');
CREATE TYPE race_ethnicity AS ENUM ('asian-indian', 'black', 'chinese', 'filipino', 'guamanian', 'hispanic', 'japanese', 'korean', 'middle-eastern', 'native-american', 'native-hawaiian', 'samoan', 'white', 'other-asian', 'other-pacific-islander', 'other');
CREATE TYPE education AS ENUM ('below-secondary', 'secondary', 'undergraduate-two-year', 'undergraduate-three-year-plus', 'graduate', 'bootcamp', 'vocational', 'other', 'non-student');

CREATE TYPE application_status AS ENUM ('pending', 'waitlisted', 'rejected', 'accepted');

CREATE TABLE applications
(
    event               text                                        not null,
    participant_id      int                                         not null,

    gender              gender                                      not null,
    race_ethnicity      race_ethnicity                              not null,
    date_of_birth       date                                        not null,

    education           education                                   not null,
    graduation_year     int                                         not null,
    major               text,

    hackathons_attended int                                         not null,
    links               text[]             default ARRAY []::text[] not null,

    address_line1       text                                        not null,
    address_line2       text,
    address_line3       text,
    locality            text,
    administrative_area text,
    postal_code         text                                        not null,
    country             text                                        not null,

    share_information   bool                                        not null,

    status              application_status default 'pending'        not null,
    flagged             bool               default false            not null,
    notes               text               default ''               not null,

    created_at          timestamp with time zone                    not null default now(),
    updated_at          timestamp with time zone                    not null default now(),

    primary key (participant_id, event)
);

CREATE TRIGGER set_applications_updated_at_timestamp
    BEFORE UPDATE
    ON applications
    FOR EACH ROW
EXECUTE PROCEDURE set_updated_at_timestamp();
