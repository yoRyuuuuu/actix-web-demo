-- Add migration script here
create table users (
  email VARCHAR NOT NULL PRIMARY KEY,
  name VARCHAR NOT NULL,
  password VARCHAR NOT NULL,
  create_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP,
  update_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP
);