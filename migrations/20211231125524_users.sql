CREATE TABLE IF NOT EXISTS users (
  id uuid PRIMARY KEY,
  username TEXT NOT NULL UNIQUE,
  password_hash TEXT NOT NULL
);
