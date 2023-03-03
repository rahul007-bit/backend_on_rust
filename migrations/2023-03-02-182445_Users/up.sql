-- create table for users
CREATE TABLE Users (
    id VARCHAR NOT NULL PRIMARY KEY UNIQUE,
    name VARCHAR NOT NULL,
    email VARCHAR NOT NULL UNIQUE,
    roll VARCHAR NOT NULL DEFAULT 'user',
    password VARCHAR NOT NULL,
    department VARCHAR NOT NULL,
    profile_image VARCHAR,
    academic_year VARCHAR NOT NULL,
);
