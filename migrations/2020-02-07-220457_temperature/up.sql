CREATE TABLE temperatures (
  id SERIAL PRIMARY KEY,
  temperature FLOAT NOT NULL,
  device VARCHAR(64) NOT NULL,
  date_recorded timestamp NOT NULL DEFAULT NOW()
)