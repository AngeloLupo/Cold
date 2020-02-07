CREATE TABLE temperature (
  id SERIAL PRIMARY KEY,
  temperature FLOAT,
  date_recorded timestamp NOT NULL DEFAULT NOW()
)