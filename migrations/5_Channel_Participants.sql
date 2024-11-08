ALTER TABLE channels ADD participants UUID ARRAY NOT NULL;
ALTER TABLE channels ADD FOREIGN KEY (EACH ELEMENT of participants) REFERENCES players(uuid);

