-- Add migration script here
create table pc (
    "id"            bigserial primary key,
    "created_at"    timestamptz(3) not null default current_timestamp,
    "updated_at"    timestamptz(3) not null default current_timestamp,
    "hostname"     varchar(60) not null,
    "key"         varchar(100) not null unique,
    "value"      varchar(150) not null
);