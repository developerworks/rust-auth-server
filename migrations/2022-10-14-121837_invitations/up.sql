CREATE TABLE invitations (
    id VARCHAR(36) NOT NULL PRIMARY KEY,
    email VARCHAR(100) NOT NULL,
    expires_at TIMESTAMP NOT NULL
);
