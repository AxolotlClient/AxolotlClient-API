-- Keeping track of the actual database schema is annoying beyond a few migrations.
-- So here is a combination of those migrations for convenience, this should be kept up to date.
-- This should not be actually used for a database, this is just a programmer reference.

-- Currently in line with: `3_Rename_show_to_public.sql`

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
	                   DEFAULT TRUE,
	retain_usernames   BOOLEAN
	                   NOT NULL
	                   DEFAULT TRUE
);

CREATE TABLE old_usernames (
	user     BLOB
	         NOT NULL,
	username VARCHAR(16) COLLATE NOCASE
	         NOT NULL,
	public   BOOLEAN
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
