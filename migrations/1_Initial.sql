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
	            NOT NULL
	            DEFAULT 'now',

	show_registered  BOOLEAN
	                 NOT NULL
	                 DEFAULT true,
	show_status      BOOLEAN
	                 NOT NULL
	                 DEFAULT true,
	retain_usernames BOOLEAN
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
	persistence_duration_seconds INT,

	FOREIGN KEY (owner) REFERENCES players(uuid) ON DELETE CASCADE
);
