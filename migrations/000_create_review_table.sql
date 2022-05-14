CREATE TABLE reviews(
  id UUID PRIMARY KEY,
  artist TEXT NOT NULL,
  release TEXT NOT NULL,
  author TEXT NOT NULL,
  year INT NOT NULL,
  article TEXT,
  created_at DATE NOT NULL,
  UNIQUE(artist, release, author)
)