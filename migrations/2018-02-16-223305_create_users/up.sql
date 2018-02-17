CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  first_name VARCHAR(128) NOT NULL,
  middle_name VARCHAR(128),
  last_name VARCHAR(128) NOT NULL,
  username VARCHAR(128) NOT NULL,
  email VARCHAR(128) NOT NULL
)
