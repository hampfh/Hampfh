-- This file should undo anything in `up.sql`
ALTER TABLE Submissions RENAME COLUMN wins TO score;