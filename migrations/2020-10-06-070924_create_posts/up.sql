-- Your SQL goes here
create table posts
(
    id         serial primary key,
    title      varchar(255)             not null,
    content    text,
    created_at timestamp with time zone not null default current_timestamp,
    updated_at timestamp with time zone not null default current_timestamp
);

alter table photos
    add column post_id int references posts (id) not null default 0;
