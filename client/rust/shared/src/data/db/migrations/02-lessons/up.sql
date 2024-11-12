CREATE TABLE IF NOT EXISTS lesson(
  id TEXT PRIMARY KEY,
  title TEXT NOT NULL,
  type INTEGER NOT NULL,
  language1 TEXT,
  language2 TEXT, 
  owner TEXT,
  updated_at INTEGER
);
