-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  uid VARCHAR(255) NOT NULL,
  username VARCHAR(255) NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE plants (
  id SERIAL PRIMARY KEY,
  uid VARCHAR(255) NOT NULL,
  symbol VARCHAR(255) NOT NULL,
  sci_name VARCHAR(255) NOT NULL,
  gener_name VARCHAR(255) NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);