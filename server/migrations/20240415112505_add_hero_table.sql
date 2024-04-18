CREATE TABLE IF NOT EXISTS heroes (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  rfid TEXT NOT NULL,
  level INTEGER NOT NULL,
  hero_type INTEGER NOT NULL,
  unallocated_skillpoints INTEGER NOT NULL,
  strength_points INTEGER NOT NULL,
  agility_points INTEGER NOT NULL,
  defence_points INTEGER NOT NULL
);