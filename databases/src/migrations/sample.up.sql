create table users (
    id SERIAL PRIMARY KEY,
    user_types integer default 0,
    name character varying(150),
    email character varying(150),
    bio text,
    is_blocked bool default false,
    blocked_reason text,
    is_deleted default false,
    updated_at timestamp without time zone default CURRENT_TIMESTAMP not null,
    created_at timestamp without time zone default CURRENT_TIMESTAMP not null
);

crete table customers ()

--\dt
--\dl
--select * from users
-- insert into users (name, email, bio, blocked_reason) values('zee', 'gh@gmail.com', 'my life', '')
--update users set bio='unhinged' where id = 1;
--update users set email='fgs@gmail.com', set bio='unhinged' where id = 1;

--update users set email='fgs@gmail.com', set bio='unhinged' where email = lower(ZEE@GMAIL.COM);
--alter table users add column password text default '' not null; 
--soft delete - bool
--hard delete-  `
--start transaction - before commiting major changes - rollback

--not case sensitive