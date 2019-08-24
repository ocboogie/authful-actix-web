-- Your SQL goes here
CREATE TABLE sessions (
  id VARCHAR(64) PRIMARY KEY,
  user_id UUID REFERENCES users(id) NOT NULL,
  expires TIMESTAMP NOT NULL
);
