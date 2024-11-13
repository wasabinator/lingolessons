CREATE TABLE IF NOT EXISTS lesson(
  id TEXT NOT NULL PRIMARY KEY,
  title TEXT NOT NULL,
  type INTEGER NOT NULL,
  language1 TEXT,
  language2 TEXT, 
  owner TEXT,
  updated_at INTEGER
);
