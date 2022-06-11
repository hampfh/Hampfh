-- Your SQL goes here
CREATE TABLE Users (
	id CHARACTER(36) PRIMARY KEY NOT NULL,
	username VARCHAR NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TABLE Submissions (
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

CREATE TABLE Matches (
	id CHARACTER(36) NOT NULL PRIMARY KEY,
	winner CHARACTER(36) NOT NULL,
	loser CHARACTER(36) NOT NULL,
	created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
	updated_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
	FOREIGN KEY (winner) REFERENCES Submissions(id),
	FOREIGN KEY (loser) REFERENCES Submissions(id)
);

CREATE TABLE Turns (
	id CHARACTER(36) NOT NULL PRIMARY KEY,
	match_id CHARACTER(36) NOT NULL,
	turn INTEGER NOT NULL,
	board TEXT NOT NULL,
	created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
	updated_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
	FOREIGN KEY (match_id) REFERENCES Matches(id)
);