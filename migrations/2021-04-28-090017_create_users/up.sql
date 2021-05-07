-- Your SQL goes here
create table users (
    id integer not null primary key,
    email varchar not null,
    password varchar not null,
    secret_2fa varchar null
)