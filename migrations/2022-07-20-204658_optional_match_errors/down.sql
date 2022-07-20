-- This file should undo anything in `up.sql`
-- Remove the field match_error from the table
CREATE TABLE Matches_OLD (
	id CHARACTER(36) NOT NULL PRIMARY KEY,
	winner CHARACTER(36) NOT NULL,
	loser CHARACTER(36) NOT NULL,
	created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
	updated_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
	p1_is_winner INTEGER NOT NULL DEFAULT 0,
	FOREIGN KEY (winner) REFERENCES Submissions(id),
	FOREIGN KEY (loser) REFERENCES Submissions(id)
);

INSERT INTO Matches_OLD SELECT id, winner, loser, created_at, updated_at, p1_is_winner FROM Matches;
DROP TABLE IF EXISTS Matches;
ALTER TABLE Matches_OLD RENAME TO Matches;