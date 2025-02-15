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

CREATE TABLE IF NOT EXISTS Solve (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    log_file TEXT,
    user_id INTEGER REFERENCES UserAccount NOT NULL,
    upload_time TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    puzzle_id INTEGER REFERENCES Puzzle NOT NULL,
    move_count INTEGER,
    uses_macros BOOLEAN NOT NULL,
    uses_filters BOOLEAN NOT NULL,
    computer_assisted BOOLEAN NOT NULL,
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
CREATE OR REPLACE VIEW InlinedSolve AS
    SELECT
        Solve.id,
        -- Solve.log_file, -- may be expensive
        (CASE
            WHEN Solve.log_file IS NULL THEN FALSE
            ELSE TRUE
        END) AS has_log_file,
        Solve.user_id,
        Solve.upload_time,
        Solve.puzzle_id,
        Solve.move_count,
        Solve.uses_macros,
        Solve.uses_filters,
        Solve.computer_assisted,
        Solve.blind,
        Solve.scramble_seed,
        Solve.program_version_id,
        Solve.log_file_verified,
        Solve.log_file_verified_by,
        Solve.solver_notes,
        Solve.moderator_notes,

        UserAccount.display_name AS user_display_name,
        ProgramVersion.program_id,
        ProgramVersion.version AS program_version,
        Program.name AS program_name,
        Program.abbreviation AS program_abbreviation,
        Puzzle.name AS puzzle_name,
        Puzzle.primary_filters AS puzzle_primary_filters,
        Puzzle.primary_macros AS puzzle_primary_macros,

        Solve.speed_cs,
        Solve.memo_cs,
        Solve.video_url,
        Solve.speed_verified,
        Solve.speed_verified_by
    FROM Solve
    LEFT JOIN UserAccount ON Solve.user_id = UserAccount.id -- must use LEFT JOIN to get join elimination
    LEFT JOIN ProgramVersion ON Solve.program_version_id = ProgramVersion.id
    LEFT JOIN Program ON ProgramVersion.program_id = Program.id
    LEFT JOIN Puzzle ON Solve.puzzle_id = Puzzle.id;

CREATE OR REPLACE VIEW VerifiedSpeedSolve AS
    SELECT * FROM InlinedSolve
    WHERE speed_verified IS TRUE AND speed_cs IS NOT NULL;

CREATE OR REPLACE VIEW VerifiedFmcSolve AS
    SELECT * FROM InlinedSolve
    WHERE log_file_verified IS TRUE AND move_count IS NOT NULL;

CREATE OR REPLACE VIEW VerifiedSpeedSolveInPrimaryCategory AS
    SELECT * FROM VerifiedSpeedSolve
    WHERE uses_filters <= puzzle_primary_filters AND uses_macros <= puzzle_primary_macros;
