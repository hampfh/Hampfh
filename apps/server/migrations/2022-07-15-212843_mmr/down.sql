-- This file should undo anything in `up.sql`
ALTER TABLE Submissions DROP COLUMN mmr;
ALTER TABLE Submissions DROP COLUMN matches_played;