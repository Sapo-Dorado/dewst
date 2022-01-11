DROP SCHEMA IF EXISTS accounts, studying CASCADE;

CREATE SCHEMA accounts;

CREATE TABLE accounts.users (
  id BIGSERIAL PRIMARY KEY,
  username VARCHAR UNIQUE NOT NULL,
  password_hash VARCHAR NOT NULL,
  token_hash VARCHAR NOT NULL,
  UNIQUE (username)
);

CREATE SCHEMA studying;

CREATE TABLE studying.decks (
  id BIGSERIAL PRIMARY KEY,
  uuid VARCHAR(36) UNIQUE NOT NULL,
  author VARCHAR,
  UNIQUE (uuid)
)