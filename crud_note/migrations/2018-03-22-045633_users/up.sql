CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  name VARCHAR(128) NOT NULL,
  identity VARCHAR(60) NOT NULL,
  hometown VARCHAR(60) NOT NULL
)