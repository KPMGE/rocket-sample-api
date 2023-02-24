CREATE TABLE IF NOT EXISTS users (
  id serial PRIMARY KEY,
  name VARCHAR (255) NOT NULL,
  age int,
  email TEXT NOT NULL,
  password TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW()
);
