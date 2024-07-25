-- Your SQL goes here
create table users (
    id SERIAL PRIMARY KEY,
    user_name character varying(150),
    email character varying(150), 
    SessionToken INTEGER
);