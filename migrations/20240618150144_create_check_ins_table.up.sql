CREATE TABLE check_ins
(
    event          text                     not null,
    participant_id int                      not null,

    at             timestamp with time zone not null default now(),

    primary key (participant_id, event)
);
