CREATE TABLE IF NOT EXISTS UserAccount (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    email VARCHAR(255),
    display_name VARCHAR(255),
    moderator BOOLEAN NOT NULL DEFAULT FALSE,
    moderator_notes TEXT NOT NULL DEFAULT '',
    dummy BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE IF NOT EXISTS Token (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    user_id INTEGER REFERENCES UserAccount NOT NULL,
    token CHAR(64) NOT NULL
);

CREATE TABLE IF NOT EXISTS Program (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    name VARCHAR(255) NOT NULL,
    abbreviation VARCHAR(255) NOT NULL
);

CREATE TABLE IF NOT EXISTS ProgramVersion (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    program_id INTEGER REFERENCES Program NOT NULL,
    version VARCHAR(31)
);

CREATE TABLE IF NOT EXISTS Puzzle (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    hsc_id VARCHAR(255),
    name VARCHAR(255) NOT NULL,
    leaderboard INTEGER -- should be another Puzzle id
);

ALTER TABLE Puzzle
    ADD CONSTRAINT fk_leaderboard FOREIGN KEY (leaderboard) REFERENCES Puzzle;

CREATE TABLE IF NOT EXISTS Solve (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    log_file TEXT,
    user_id INTEGER REFERENCES UserAccount NOT NULL,
    upload_time TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    puzzle_id INTEGER REFERENCES Puzzle NOT NULL,
    move_count INTEGER,
    uses_macros BOOLEAN NOT NULL,
    uses_filters BOOLEAN NOT NULL,
    speed_cs INTEGER,
    memo_cs INTEGER,
    blind BOOLEAN NOT NULL,
    scramble_seed CHAR(64),
    program_version_id INTEGER REFERENCES ProgramVersion, -- NULL should mean "unknown"
    speed_evidence_id INTEGER DEFAULT NULL, -- points to the canonical evidence
    valid_solve BOOLEAN, -- NULL should mean "unverifiable" or "not yet verified", FALSE is "invalid log"
    solver_notes TEXT NOT NULL DEFAULT '',
    moderator_notes TEXT NOT NULL DEFAULT '',
    rank INTEGER -- do not assign to this
);

CREATE TABLE IF NOT EXISTS SpeedEvidence (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    solve_id INTEGER REFERENCES Solve NOT NULL,
    video_url TEXT,
    verified BOOLEAN, -- NULL should mean "not yet verified", FALSE is "invalid evidence"
    verified_by INTEGER REFERENCES UserAccount NOT NULL,
    moderator_notes TEXT NOT NULL DEFAULT ''
);

ALTER TABLE Solve
    ADD CONSTRAINT fk_speed_evidence_id FOREIGN KEY (speed_evidence_id) REFERENCES SpeedEvidence;

CREATE TABLE DiscordConnection (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    user_id INTEGER REFERENCES UserAccount NOT NULL,
    discord_id BIGINT NOT NULL,
    discord_verified BOOLEAN NOT NULL DEFAULT FALSE
);


/*
CREATE OR REPLACE TRIGGER update_solve_rankings
    AFTER INSERT ON Solve
    FOR EACH ROW
    EXECUTE FUNCTION update_solve_rankings_row();

CREATE FUNCTION update_solve_rankings_row() RETURNS void AS $$
DECLARE
    prev_best := SELECT * FROM Solve
        WHERE rank IS NOT NULL
            AND user_id = NEW.user_id
            AND blind = NEW.blind
            AND (NOT (uses_filters AND NEW.uses_filters))
            AND (NOT (uses_macros AND NEW.uses_macros))
        LIMIT 1;
BEGIN
    IF prev_best 
END
$$ LANGUAGE plpgsql
*/
