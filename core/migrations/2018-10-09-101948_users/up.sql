-- Your SQL goes here
CREATE TABLE users (
  id UUID PRIMARY KEY,
  email VARCHAR(100) NOT NULL UNIQUE,
  password VARCHAR(122) NOT NULL, --argon hash
  created_at TIMESTAMP NOT NULL
);
