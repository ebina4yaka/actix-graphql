-- This file should undo anything in `up.sql`
alter table photos
    drop column post_id;
drop table posts
