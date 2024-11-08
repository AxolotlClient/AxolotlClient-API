CREATE TABLE channel_memberships (
    player UUID           NOT NULL,
    channels BIGINT ARRAY NOT NULL,

    PRIMARY KEY (player),
    FOREIGN KEY (player) REFERENCES players(uuid) ON DELETE CASCADE
);
