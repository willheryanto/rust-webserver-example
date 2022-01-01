CREATE TABLE IF NOT EXISTS movies (
  id BIGINT PRIMARY KEY,
  title TEXT NOT NULL,
  image_url TEXT NOT NULL,
  overview TEXT NOT NULL,
  release_date DATE NOT NULL,
  vote_average DOUBLE PRECISION NOT NULL
);

