-- Keeping track of the actual database schema is annoying beyond a few migrations.
-- So here is a combination of those migrations for convenience, this should be kept up to date.
-- This should not be actually used for a database, this is just a programmer reference.

-- Currently in line with: `migrations/3_Relations.sql`

CREATE TABLE players (
	uuid     UUID
	         PRIMARY KEY,
	username VARCHAR(16)
	         NOT NULL
	         UNIQUE,

	registered  TIMESTAMP
	            NOT NULL
	            DEFAULT 'now',
	last_online TIMESTAMP
	            DEFAULT 'now',

	show_registered  BOOLEAN
	                 NOT NULL
	                 DEFAULT true,
	retain_usernames BOOLEAN
	                 NOT NULL
	                 DEFAULT true,
	show_last_online BOOLEAN
	                 NOT NULL
	                 DEFAULT true,
	show_activity    BOOLEAN
	                 NOT NULL
	                 DEFAULT true
);

CREATE TABLE previous_usernames (
	player   UUID
	         NOT NULL,
	username VARCHAR(16)
	         NOT NULL,
	public   BOOLEAN
	         NOT NULL
	         DEFAULT TRUE,

	FOREIGN KEY (player) REFERENCES players(uuid) ON DELETE CASCADE
);

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

CREATE TABLE tokens (
	token   BYTEA
	        PRIMARY KEY,
	player  UUID,

	created TIMESTAMP
	        NOT NULL
	        CHECK (used >= created)
	        DEFAULT 'now',
	used    TIMESTAMP
	        NOT NULL
	        CHECK (used >= created)
	        DEFAULT 'now',

	revoked BOOLEAN
	        NOT NULL
	        DEFAULT false,
	expired BOOLEAN
	        NOT NULL
	        GENERATED ALWAYS AS (used - created > '1 day') STORED,
	valid   BOOLEAN
	        NOT NULL
	        GENERATED ALWAYS AS (player IS NOT NULL AND NOT revoked AND NOT used - created > '1 day') STORED,

	FOREIGN KEY (player) REFERENCES players(uuid) ON DELETE SET NULL
);

CREATE TABLE channels (
	id                           BIGINT
	                             PRIMARY KEY,
	name                         VARCHAR(32)
	                             NOT NULL
	                             UNIQUE,
	owner                        UUID
	                             NOT NULL,

	created                      TIMESTAMP
	                             NOT NULL
	                             DEFAULT 'now',
	last_updated                 TIMESTAMP
								NOT NULL
	                             DEFAULT 'now',
	last_message                 TIMESTAMP
	                             NOT NULL
	                             DEFAULT 'now',

	persistence                  SMALLINT
	                             NOT NULL,
	persistence_count            INT,
	persistence_duration_seconds BIGINT,

	FOREIGN KEY (owner) REFERENCES players(uuid) ON DELETE CASCADE
);

CREATE TABLE channel_memberships (
	player		UUID NOT NULL,
	channels 	BIGINT ARRAY NOT NULL,

	PRIMARY KEY (player),
	FOREIGN KEY (player) REFERENCES players(uuid) ON DELETE CASCADE
);
