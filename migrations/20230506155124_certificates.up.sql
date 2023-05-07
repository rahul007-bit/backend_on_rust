-- Add up migration script here
-- create table for certificate
CREATE TABLE Certificates (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    description VARCHAR NOT NULL,
    -- here id is a serial type
    created_by_id INTEGER NOT NULL,
    FOREIGN KEY (created_by_id) REFERENCES users(id)
);
