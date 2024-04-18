CREATE TABLE IF NOT EXISTS heroes (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  rfid TEXT,
  level INTEGER,
  hero_type INTEGER,
  unallocated_skillpoints INTEGER,
  strength_points INTEGER,
  agility_points INTEGER,
  defence_points INTEGER
);