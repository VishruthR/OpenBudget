CREATE TABLE IF NOT EXISTS transaction_category_rule (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    category_id INTEGER NOT NULL,
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (category_id) REFERENCES category(id) ON DELETE CASCADE
);
