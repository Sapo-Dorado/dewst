DROP SCHEMA IF EXISTS accounts CASCADE;
CREATE SCHEMA accounts;

CREATE TABLE accounts.users (
  id BIGSERIAL PRIMARY KEY,
  username VARCHAR UNIQUE NOT NULL,
  password_hash VARCHAR NOT NULL,
  token_hash VARCHAR NOT NULL,
  UNIQUE (username)
);