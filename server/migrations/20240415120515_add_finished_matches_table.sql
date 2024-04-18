CREATE TABLE IF NOT EXISTS finished_matches (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  hero_1_id INTEGER,
  hero_2_id INTEGER,
  winner_hero_id INTEGER NULL,
  FOREIGN KEY(hero_1_id) REFERENCES heroes(id),
  FOREIGN KEY(hero_2_id) REFERENCES heroes(id),
  FOREIGN KEY(winner_hero_id) REFERENCES heroes(id)
);