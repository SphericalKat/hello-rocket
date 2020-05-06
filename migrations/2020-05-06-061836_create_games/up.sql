CREATE TABLE IF NOT EXISTS games
(
    id        SERIAL PRIMARY KEY,
    name      TEXT    NOT NULL,
    developer TEXT    NOT NULL,
    is_goty   BOOLEAN NOT NULL DEFAULT 'f'
)