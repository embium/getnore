-- Add migration script here
CREATE TABLE billing (
    id VARCHAR(255) PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    price_id TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ,
    deleted_at TIMESTAMPTZ
);
