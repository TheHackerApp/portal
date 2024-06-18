-- rename the existing type so we can re-create it
ALTER TYPE race_ethnicity RENAME TO old_race_ethnicity;

-- re-create the type
CREATE TYPE race_ethnicity AS ENUM ('asian-indian', 'black', 'chinese', 'filipino', 'guamanian', 'hispanic', 'japanese', 'korean', 'middle-eastern', 'native-american', 'native-hawaiian', 'samoan', 'white', 'other-asian', 'other-pacific-islander', 'other');

-- update the columns
ALTER TABLE applications
    ALTER COLUMN race_ethnicity TYPE race_ethnicity USING race_ethnicity::text::race_ethnicity;
ALTER TABLE draft_applications
    ALTER COLUMN race_ethnicity TYPE race_ethnicity USING race_ethnicity::text::race_ethnicity;

-- delete the existing type
DROP TYPE old_race_ethnicity;
