-- Add migration script here
CREATE TABLE search_history (
    id SERIAL PRIMARY KEY,
    query TEXT NOT NULL,
    found BOOLEAN NOT NULL,
    searched_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);