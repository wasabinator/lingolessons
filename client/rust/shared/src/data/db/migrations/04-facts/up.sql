CREATE TABLE IF NOT EXISTS fact(
  id BLOB NOT NULL PRIMARY KEY,
  lesson_id BLOB NOT NULL,
  element1 TEXT,
  element2 TEXT,
  hint TEXT,
  updated_at INTEGER,
  FOREIGN KEY(lesson_id) REFERENCES lesson(id) ON DELETE CASCADE
);
