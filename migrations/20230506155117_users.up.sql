-- Add up migration script here
-- create table for users
CREATE TABLE Users (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    email VARCHAR NOT NULL UNIQUE,
    -- allowed roles are: admin, user, student
    role VARCHAR NOT NULL DEFAULT 'student' CHECK (role IN ('admin', 'user', 'student','hod','accountant')),
    password VARCHAR NOT NULL,
    department VARCHAR NOT NULL,
    profile_image VARCHAR,
    academic_year VARCHAR NOT NULL
);
