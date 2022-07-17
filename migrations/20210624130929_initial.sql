-- Add migration script here
CREATE TABLE IF NOT EXISTS clips (
    id text primary key NOT NULL,
    title text,
    content text NOT NULL,
    shortcode text UNIQUE NOT NULL,
    created_at datetime NOT NULL,
    expires_at datetime,
    password text,
    hits bigint NOT NULL
);
