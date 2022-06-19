-- This file should undo anything in `up.sql`

-- We remove the disqualification field by
-- creating a new table without it and 
-- copying the data over.

CREATE TABLE Submissions_OLD (
	id CHARACTER(36) NOT NULL PRIMARY KEY,
	user CHARACTER(36) NOT NULL,
	script TEXT NOT NULL,
	comment TEXT,
	score INTEGER NOT NULL DEFAULT 0,
	issue_url TEXT NOT NULL,
	issue_number INTEGER NOT NULL,
	created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
	FOREIGN KEY (user) REFERENCES Users(id)
);

INSERT INTO Submissions_OLD SELECT id, user, script, comment, score, issue_url, issue_number, created_at, updated_at FROM Submissions;
DROP TABLE IF EXISTS Submissions;
ALTER TABLE Submissions_OLD RENAME TO Submissions