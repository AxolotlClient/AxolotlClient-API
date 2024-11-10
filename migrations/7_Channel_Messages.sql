CREATE TABLE messages(
    id          BIGINT 
                NOT NULL
                PRIMARY KEY UNIQUE,
    channel_id  BIGINT 
                NOT NULL,
    sender      UUID NOT NULL,
    -- sender_name is used for proxying. 
    -- The limit is a result from PluralKit's limits: https://pluralkit.me/api/models/
    sender_name VARCHAR(179) NOT NULL,
    content     VARCHAR(2000) NOT NULL,
    send_time   TIMESTAMP NOT NULL DEFAULT LOCALTIMESTAMP,

    FOREIGN KEY (channel_id) REFERENCES channels(id) ON DELETE CASCADE,
    FOREIGN KEY (sender) REFERENCES players(uuid) ON DELETE CASCADE
);