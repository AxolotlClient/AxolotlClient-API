ALTER TABLE tokens ALTER created SET DEFAULT LOCALTIMESTAMP;
ALTER TABLE tokens ALTER used SET DEFAULT LOCALTIMESTAMP;

ALTER TABLE players ALTER registered SET DEFAULT LOCALTIMESTAMP;
ALTER TABLE players ALTER last_online SET DEFAULT LOCALTIMESTAMP;

ALTER TABLE channels ALTER created SET DEFAULT LOCALTIMESTAMP;
ALTER TABLE channels ALTER last_updated SET DEFAULT LOCALTIMESTAMP;
ALTER TABLE channels ALTER last_message SET DEFAULT LOCALTIMESTAMP;
