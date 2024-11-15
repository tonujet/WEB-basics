CREATE TABLE IF NOT EXISTS cars (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    production_date TEXT NOT NULL,
    brand TEXT NOT NULL,
    color TEXT NOT NULL,
    state TEXT NOT NULL,
    owner_name TEXT NOT NULL
);