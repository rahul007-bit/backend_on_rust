-- Add up migration script here
CREATE TABLE Questions (
    id SERIAL PRIMARY KEY,
    certificate_id INTEGER NOT NULL,
    question VARCHAR(255) NOT NULL,
    -- question_id VARCHAR(255) NOT NULL,
    options VARCHAR Array,
    question_type VARCHAR(255) NOT NULL,
    -- checkbox can be array of strings or none
    checkbox VARCHAR Array,
    drop_down VARCHAR Array,
    is_required BOOLEAN  NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    FOREIGN KEY (certificate_id) REFERENCES Certificates(id) ON DELETE CASCADE
);
