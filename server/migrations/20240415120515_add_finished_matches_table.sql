CREATE TABLE IF NOT EXISTS finished_matches (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  hero_1_id INTEGER NOT NULL,
  hero_2_id INTEGER NOT NULL,
  winner_hero_id INTEGER,
  FOREIGN KEY(hero_1_id) REFERENCES heroes(id),
  FOREIGN KEY(hero_2_id) REFERENCES heroes(id),
  FOREIGN KEY(winner_hero_id) REFERENCES heroes(id)
);