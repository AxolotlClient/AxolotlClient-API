CREATE TABLE channel_invites (
    channel     BIGINT NOT NULL,
    player      UUID NOT NULL,
    sender      UUID NOT NULL,
    
    UNIQUE      (channel, player),

    FOREIGN KEY (sender) REFERENCES players(uuid) ON DELETE CASCADE,
    FOREIGN KEY (player) REFERENCES players(uuid) ON DELETE CASCADE,
    FOREIGN KEY (channel) REFERENCES channels(id) ON DELETE CASCADE
);
