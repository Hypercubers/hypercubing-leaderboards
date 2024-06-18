CREATE TABLE IF NOT EXISTS UserAccount (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    email VARCHAR(255) NOT NULL,
    display_name VARCHAR(255),
    moderator BOOLEAN NOT NULL DEFAULT FALSE
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
    version VARCHAR(31) NOT NULL
);

CREATE TABLE IF NOT EXISTS Puzzle (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    hsc_id VARCHAR(255),
    leaderboard INTEGER -- should be another Puzzle id
);

ALTER TABLE Puzzle
    ADD CONSTRAINT fk_leaderboard FOREIGN KEY (leaderboard) REFERENCES Puzzle;

CREATE TABLE IF NOT EXISTS PuzzleVersion (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    puzzle_id INTEGER REFERENCES Puzzle NOT NULL,
    version VARCHAR(31) NOT NULL
);

CREATE TABLE IF NOT EXISTS Solve (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    log_file TEXT,
    user_id INTEGER REFERENCES UserAccount NOT NULL,
    upload_time TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    puzzle_version_id INTEGER REFERENCES PuzzleVersion NOT NULL,
    move_count INTEGER NOT NULL,
    uses_macros BOOLEAN NOT NULL,
    uses_filters BOOLEAN NOT NULL,
    speed_cs INTEGER,
    memo_cs INTEGER,
    blind BOOLEAN NOT NULL,
    scramble_seed CHAR(64),
    program_version_id INTEGER REFERENCES ProgramVersion,
    speed_evidence_id INTEGER DEFAULT NULL, -- points to the canonical evidence
    valid_solve BOOLEAN NOT NULL
);

CREATE TABLE IF NOT EXISTS SpeedEvidence (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    solve_id INTEGER REFERENCES Solve NOT NULL,
    video_url TEXT,
    verified BOOLEAN, -- NULL should mean "not yet verified", FALSE is "invalid evidence"
    verified_by INTEGER REFERENCES UserAccount NOT NULL
);

ALTER TABLE Solve
    ADD CONSTRAINT fk_speed_evidence_id FOREIGN KEY (speed_evidence_id) REFERENCES SpeedEvidence;

