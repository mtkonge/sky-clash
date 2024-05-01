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

CREATE TABLE IF NOT EXISTS finished_matches (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  hero_1_id INTEGER NOT NULL,
  hero_2_id INTEGER NOT NULL,
  winner_hero_id INTEGER,
  FOREIGN KEY(hero_1_id) REFERENCES heroes(id),
  FOREIGN KEY(hero_2_id) REFERENCES heroes(id),
  FOREIGN KEY(winner_hero_id) REFERENCES heroes(id)
);
