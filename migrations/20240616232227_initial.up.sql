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
    token CHAR(64) NOT NULL,
    expiry TIMESTAMPTZ NOT NULL
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
    log_file_verified BOOLEAN,
    log_file_verified_by INTEGER REFERENCES UserAccount,
    solver_notes TEXT NOT NULL DEFAULT '',
    moderator_notes TEXT NOT NULL DEFAULT '',

    speed_cs INTEGER,
    memo_cs INTEGER,
    video_url TEXT,
    speed_verified BOOLEAN,
    speed_verified_by INTEGER REFERENCES UserAccount
);

-- the moderator or uploader view of the solve
CREATE OR REPLACE VIEW FullSolve AS
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
        Solve.log_file_verified,
        Solve.solver_notes,
        UserAccount.display_name,
        ProgramVersion.program_id,
        ProgramVersion.version,
        Program.name AS program_name,
        Program.abbreviation,
        Puzzle.name AS puzzle_name,
        Puzzle.primary_filters,
        Puzzle.primary_macros,
        Solve.speed_cs,
        Solve.memo_cs,
        Solve.video_url,
        Solve.speed_verified
    FROM Solve
    LEFT JOIN UserAccount ON Solve.user_id = UserAccount.id -- must use LEFT JOIN to get join elimination
    LEFT JOIN ProgramVersion ON Solve.program_version_id = ProgramVersion.id
    LEFT JOIN Program ON ProgramVersion.program_id = Program.id
    LEFT JOIN Puzzle ON Solve.puzzle_id = Puzzle.id;

-- the public view of the solve
CREATE OR REPLACE VIEW LeaderboardSolve AS
    SELECT
        id,
        (CASE WHEN log_file_verified IS TRUE THEN log_file ELSE NULL END) AS log_file,
        user_id,
        upload_time,
        puzzle_id,
        (CASE WHEN log_file_verified IS TRUE THEN move_count ELSE NULL END) AS move_count,
        uses_macros,
        uses_filters,
        blind,
        scramble_seed,
        program_version_id,
        log_file_verified,
        solver_notes,
        display_name,
        program_id,
        version,
        program_name,
        abbreviation,
        puzzle_name,
        primary_filters,
        primary_macros,
        (CASE WHEN speed_verified IS TRUE THEN speed_cs ELSE NULL END) AS speed_cs,
        (CASE WHEN speed_verified IS TRUE THEN memo_cs ELSE NULL END) AS memo_cs,
        (CASE WHEN speed_verified IS TRUE THEN video_url ELSE NULL END) AS video_url,
        speed_verified
    FROM FullSolve
    WHERE speed_verified IS TRUE
        OR (speed_verified IS NULL AND log_file_verified IS TRUE AND NOT blind);


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
