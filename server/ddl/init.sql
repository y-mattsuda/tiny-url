CREATE TABLE url (
    id CHAR(36) PRIMARY KEY,
    `long` VARCHAR(255) NOT NULL,
    short CHAR(7) NOT NULL,
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL
) CHARACTER SET utf8mb4 COLLATE utf8mb4_bin;
