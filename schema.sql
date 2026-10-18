CREATE TABLE IF NOT EXISTS user_counters (
  day TEXT NOT NULL,
  device_id TEXT NOT NULL,
  count INTEGER NOT NULL DEFAULT 0,
  PRIMARY KEY (day, device_id)
);

CREATE TABLE IF NOT EXISTS global_counters (
  day TEXT PRIMARY KEY,
  count INTEGER NOT NULL DEFAULT 0
);
