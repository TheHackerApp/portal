CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE schools
(
    id   uuid primary key not null default uuid_generate_v4(),
    name text             not null
);
