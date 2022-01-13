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
  title VARCHAR NOT NULL,
  UNIQUE (uuid)
);

CREATE TABLE studying.cards (
  id BIGSERIAL PRIMARY KEY,
  deck_uuid VARCHAR(36) REFERENCES studying.decks(uuid),
  front VARCHAR,
  back VARCHAR
);