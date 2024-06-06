ALTER TABLE players ALTER last_online DROP NOT NULL;

ALTER TABLE players ADD show_last_online BOOLEAN
                                         NOT NULL
                                         DEFAULT true;

ALTER TABLE players ADD show_activity BOOLEAN
                                      NOT NULL
                                      DEFAULT true;

UPDATE players SET show_last_online = show_status AND show_activity = show_status;

