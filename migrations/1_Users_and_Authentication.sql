CREATE TABLE users (
	uuid               BLOB
	                   PRIMARY KEY
	                   NOT NULL
	                   UNIQUE,
	username           VARCHAR(16) COLLATE NOCASE
	                   NOT NULL
	                   UNIQUE,
	registered         DATETIME
	                   NOT NULL
	                   DEFAULT CURRENT_TIMESTAMP,
	show_registered    BOOLEAN
	                   NOT NULL
	                   DEFAULT TRUE,
	last_activity      DATETIME
	                   NOT NULL
	                   DEFAULT CURRENT_TIMESTAMP,
	show_last_activity BOOLEAN
	                   NOT NULL
	                   DEFAULT TRUE
);

CREATE TABLE old_usernames (
	user     BLOB
	         NOT NULL,
	username VARCHAR(16) COLLATE NOCASE
	         NOT NULL,
	show     BOOLEAN
	         NOT NULL
	         DEFAULT TRUE,

	FOREIGN KEY (user) REFERENCES users(uuid) ON DELETE CASCADE
);

CREATE TABLE tokens (
	token   BLOB
	        PRIMARY KEY
	        NOT NULL
	        UNIQUE,
	created DATETIME
	        NOT NULL
	        CHECK (used >= created)
	        DEFAULT CURRENT_TIMESTAMP,
	used    DATETIME
	        NOT NULL
	        CHECK (used >= created)
	        DEFAULT CURRENT_TIMESTAMP,
	revoked BOOLEAN
	        NOT NULL
	        DEFAULT FALSE,
	expired BOOLEAN
	        NOT NULL
	        GENERATED ALWAYS AS (unixepoch(used) - unixepoch(created) > 60 * 60 * 24),
	valid   BOOLEAN
	        NOT NULL
	        GENERATED ALWAYS AS (user IS NOT NULL AND NOT revoked AND NOT expired),
	user    BLOB,

	FOREIGN KEY (user) REFERENCES users(uuid) ON DELETE SET NULL
);
