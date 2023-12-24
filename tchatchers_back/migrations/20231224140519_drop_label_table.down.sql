-- Add down migration script here
CREATE TABLE LABEL (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL UNIQUE,
    default_translation VARCHAR NOT NULL
);