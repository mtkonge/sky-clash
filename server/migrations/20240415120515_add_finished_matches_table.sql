CREATE TABLE IF NOT EXISTS finished_matches (
  id INT PRIMARY KEY AUTOINCREMENT,
  RFID TEXT,
  level INT,
  hero_type: INT,
  unallocated_skillpoints: INT,
  strength_points: INT,
  agility_points: INT,
  defence_points: INT
);