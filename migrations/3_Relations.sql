CREATE TYPE relation AS ENUM (
	'blocked',
	'none', -- This relation should never appear in the database, it is here
	        -- so that SQLx can properly map Rust types to PostgreSQL types
	'request',
	'friend'
);

CREATE TABLE relations (
	player_a UUID     NOT NULL,
	player_b UUID     NOT NULL,
	relation RELATION NOT NULL,

	PRIMARY KEY (player_a, player_b),

	FOREIGN KEY (player_a) REFERENCES players(uuid) ON DELETE CASCADE,
	FOREIGN KEY (player_b) REFERENCES players(uuid) ON DELETE CASCADE
);
