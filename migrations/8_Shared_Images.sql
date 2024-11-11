CREATE TABLE images (
    id          BIGINT 
                NOT NULL 
                PRIMARY KEY
                UNIQUE,
    player      UUID
                NOT NULL,
    filename    BYTEA NOT NULL,
    file        BYTEA NOT NULL,
    timestamp   TIMESTAMP NOT NULL DEFAULT LOCALTIMESTAMP,

    FOREIGN KEY (player) REFERENCES players(uuid) ON DELETE CASCADE
);