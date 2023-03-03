-- create table for questions
CREATE TABLE Question (
    id SERIAL PRIMARY KEY,
    certificate_id VARCHAR NOT NULL,
    question VARCHAR(255) NOT NULL,
    -- question_id VARCHAR(255) NOT NULL,
    options TEXT[] ,
    question_type VARCHAR(255) ,
    checkbox TEXT[] ,
    drop_down TEXT[] ,
    is_required BOOLEAN ,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    FOREIGN KEY (certificate_id) REFERENCES Certificates(id) ON DELETE CASCADE
);
