CREATE TABLE IF NOT EXISTS UserAccount (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    email VARCHAR(255),
    discord_id BIGINT,
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
    name VARCHAR(255) NOT NULL,
    primary_filters BOOLEAN NOT NULL, -- whether the primary category uses filters
    primary_macros BOOLEAN NOT NULL -- whether the primary category uses macros
);

CREATE TABLE IF NOT EXISTS HscPuzzle (
    hsc_id VARCHAR(255) PRIMARY KEY,
    puzzle_id INTEGER REFERENCES Puzzle NOT NULL
);

-- TODO: move speed to SpeedEvidence
CREATE TABLE IF NOT EXISTS Solve (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    log_file TEXT,
    user_id INTEGER REFERENCES UserAccount NOT NULL,
    upload_time TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    puzzle_id INTEGER REFERENCES Puzzle NOT NULL,
    move_count INTEGER,
    uses_macros BOOLEAN NOT NULL,
    uses_filters BOOLEAN NOT NULL,
    blind BOOLEAN NOT NULL,
    scramble_seed CHAR(64),
    program_version_id INTEGER REFERENCES ProgramVersion NOT NULL,
    speed_evidence_id INTEGER, -- points to the canonical evidence
    valid_log_file BOOLEAN, -- NULL should mean "unverifiable" or "not yet verified", FALSE is "invalid log"
    solver_notes TEXT NOT NULL DEFAULT '',
    moderator_notes TEXT NOT NULL DEFAULT ''
);

CREATE TABLE IF NOT EXISTS SpeedEvidence (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    solve_id INTEGER REFERENCES Solve NOT NULL,
    speed_cs INTEGER,
    memo_cs INTEGER,
    video_url TEXT,
    verified BOOLEAN, -- NULL should mean "not yet verified", FALSE is "invalid evidence"
    verified_by INTEGER REFERENCES UserAccount,
    moderator_notes TEXT NOT NULL DEFAULT ''
);

ALTER TABLE Solve
    ADD CONSTRAINT fk_speed_evidence_id FOREIGN KEY (speed_evidence_id) REFERENCES SpeedEvidence;

CREATE OR REPLACE VIEW LeaderboardSolve AS
    SELECT
        Solve.id,
        Solve.log_file,
        Solve.user_id,
        Solve.upload_time,
        Solve.puzzle_id,
        Solve.move_count,
        Solve.uses_macros,
        Solve.uses_filters,
        Solve.blind,
        Solve.scramble_seed,
        Solve.program_version_id,
        Solve.speed_evidence_id,
        Solve.valid_log_file,
        Solve.solver_notes,
        UserAccount.display_name,
        ProgramVersion.program_id,
        ProgramVersion.version,
        Program.name AS program_name,
        Program.abbreviation,  
        Puzzle.name AS puzzle_name,
        Puzzle.primary_filters,
        Puzzle.primary_macros,
        SpeedEvidence.speed_cs,
        SpeedEvidence.memo_cs,
        SpeedEvidence.video_url,
        SpeedEvidence.verified
    FROM Solve
    LEFT JOIN UserAccount ON Solve.user_id = UserAccount.id -- must use LEFT JOIN to get join elimination
    LEFT JOIN ProgramVersion ON Solve.program_version_id = ProgramVersion.id
    LEFT JOIN Program ON ProgramVersion.program_id = Program.id
    LEFT JOIN Puzzle ON Solve.puzzle_id = Puzzle.id
    LEFT JOIN SpeedEvidence ON SpeedEvidence.id = Solve.speed_evidence_id
    WHERE
        (Solve.log_file IS NULL AND SpeedEvidence.verified)
        OR Solve.valid_log_file;


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
