CREATE TABLE IF NOT EXISTS setting(
    key TEXT NOT NULL PRIMARY KEY,
    value TEXT,
    created INTEGER DEFAULT (CAST(strftime('%s', 'now') AS INTEGER)),
    modified INTEGER DEFAULT (CAST(strftime('%s', 'now') AS INTEGER))
);

CREATE TABLE IF NOT EXISTS token(
  id INTEGER NOT NULL PRIMARY KEY DEFAULT 1,
  username TEXT NOT NULL,
  authToken TEXT NOT NULL,
  refreshToken TEXT NOT NULL
);
