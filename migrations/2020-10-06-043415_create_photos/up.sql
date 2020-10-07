-- Your SQL goes here
create table photos
(
    id          serial                   primary key ,
    name        varchar(255)             not null,
    description text,
    created_at  timestamp with time zone not null default current_timestamp,
    updated_at  timestamp with time zone not null default current_timestamp
);
