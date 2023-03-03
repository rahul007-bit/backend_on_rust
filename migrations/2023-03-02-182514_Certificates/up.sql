-- create table for certificate
CREATE TABLE Certificates (
    id VARCHAR NOT NULL PRIMARY KEY UNIQUE,
    name VARCHAR NOT NULL,
    description VARCHAR NOT NULL,
    created_by_id VARCHAR NOT NULL,
    FOREIGN KEY (created_by_id) REFERENCES users(id),
);
