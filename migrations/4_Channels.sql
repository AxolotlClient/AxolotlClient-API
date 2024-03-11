CREATE TABLE channels (
	id                           UNSIGNED BIGINT
	                             PRIMARY KEY
	                             NOT NULL
	                             UNIQUE,
	name                         VARCHAR(32) COLLATE NOCASE
	                             NOT NULL
	                             UNIQUE,
	owner                        BLOB
	                             NOT NULL,
	created                      DATETIME
	                             NOT NULL
	                             DEFAULT CURRENT_TIMESTAMP,
	last_updated                 DATETIME
                                 NOT NULL
                                 DEFAULT CURRENT_TIMESTAMP,
	last_active                  DATETIME
                                 NOT NULL
                                 DEFAULT CURRENT_TIMESTAMP,
	persistence                  UNSIGNED TINYINT
	                             NOT NULL,
	persistence_count            UNSIGNED INT,
	persistence_duration_seconds UNSIGNED INT,

	FOREIGN KEY (owner) REFERENCES users (uuid) ON DELETE CASCADE
);
