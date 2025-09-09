CREATE TABLE IF NOT EXISTS UserAccount (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,

    name VARCHAR(255),
    moderator_notes TEXT NOT NULL DEFAULT '',

    -- Contact
    email VARCHAR(255),
    discord_id BIGINT,

    -- Flags
    moderator BOOLEAN NOT NULL DEFAULT FALSE,
    dummy BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE IF NOT EXISTS Token (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,

    user_id INTEGER REFERENCES UserAccount ON DELETE CASCADE NOT NULL,
    string CHAR(64) NOT NULL,
    expiry TIMESTAMPTZ NOT NULL
);

CREATE TABLE IF NOT EXISTS Program (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,

    name VARCHAR(255) NOT NULL,
    abbr VARCHAR(255) NOT NULL,

    material BOOLEAN NOT NULL
);

CREATE TABLE IF NOT EXISTS Variant (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,

    name VARCHAR(255) NOT NULL,
    prefix VARCHAR(255) NOT NULL,
    suffix VARCHAR(255) NOT NULL,
    abbr VARCHAR(255) NOT NULL,

    material_by_default BOOLEAN NOT NULL,
    primary_filters BOOLEAN NOT NULL, -- whether the variant allows filters by default
    primary_macros BOOLEAN NOT NULL -- whether the variant allows macros by default
);

CREATE TABLE IF NOT EXISTS Puzzle (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,

    name VARCHAR(255) NOT NULL,
    primary_filters BOOLEAN NOT NULL, -- whether the default variant allows filters by default
    primary_macros BOOLEAN NOT NULL -- whether the default variant allows macros by default
);

CREATE TABLE IF NOT EXISTS HscPuzzle (
    hsc_id VARCHAR(255) PRIMARY KEY,
    puzzle_id INTEGER REFERENCES Puzzle ON DELETE CASCADE NOT NULL
);

CREATE TABLE IF NOT EXISTS SpecialEvent (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,

    name VARCHAR(255) NOT NULL,
    query TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS Solve (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,

    -- Metadata
    solver_id INTEGER REFERENCES UserAccount NOT NULL,
    solve_date TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    upload_date TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    solver_notes TEXT NOT NULL DEFAULT '',
    moderator_notes TEXT NOT NULL DEFAULT '',

    -- Event
    puzzle_id INTEGER REFERENCES Puzzle NOT NULL,
    variant_id INTEGER REFERENCES Variant,
    program_id INTEGER REFERENCES Program NOT NULL,
    average BOOLEAN NOT NULL,
    blind BOOLEAN NOT NULL,
    filters BOOLEAN NOT NULL,
    macros BOOLEAN NOT NULL,
    one_handed BOOLEAN NOT NULL,
    computer_assisted BOOLEAN NOT NULL,

    -- Score
    move_count INTEGER,
    speed_cs INTEGER,
    memo_cs INTEGER,

    -- Verification
    fmc_verified BOOLEAN,
    fmc_verified_by INTEGER REFERENCES UserAccount,
    speed_verified BOOLEAN,
    speed_verified_by INTEGER REFERENCES UserAccount,

    -- Evidence
    log_file_name TEXT,
    log_file_contents BYTEA,
    scramble_seed CHAR(64),
    video_url TEXT
);

CREATE OR REPLACE VIEW InlinedSolve AS
    SELECT
        Solve.id,

        -- Metadata
        Solve.solve_date,
        Solve.upload_date,
        Solve.solver_notes,
        Solve.moderator_notes,

        -- Flags
        Solve.average,
        Solve.blind,
        Solve.filters,
        Solve.macros,
        Solve.one_handed,
        Solve.computer_assisted,

        -- Score
        Solve.move_count,
        Solve.speed_cs,
        Solve.memo_cs,

        -- Verification
        Solve.fmc_verified,
        Solve.fmc_verified_by,
        Solve.speed_verified,
        Solve.speed_verified_by,

        -- Evidence
        (CASE WHEN Solve.log_file_name IS NULL THEN FALSE ELSE TRUE END) as has_log_file, -- log file may be too big
        Solve.scramble_seed,
        Solve.video_url,

        -- Puzzle
        Puzzle.id AS puzzle_id,
        Puzzle.name AS puzzle_name,
        Puzzle.primary_filters AS puzzle_primary_filters,
        Puzzle.primary_macros AS puzzle_primary_macros,

        -- Variant
        Variant.id AS variant_id,
        Variant.name AS variant_name,
        Variant.prefix AS variant_prefix,
        Variant.suffix AS variant_suffix,
        Variant.abbr AS variant_abbr,
        Variant.material_by_default AS variant_material_by_default,
        Variant.primary_filters AS variant_primary_filters,
        Variant.primary_macros AS variant_primary_macros,

        COALESCE(variant.primary_filters, puzzle.primary_filters) AS primary_filters,
        COALESCE(variant.primary_macros, puzzle.primary_macros) AS primary_macros,

        -- Program
        Program.id AS program_id,
        Program.name AS program_name,
        Program.abbr AS program_abbr,
        Program.material AS program_material,

        -- Solver
        Solve.solver_id,
        UserAccount.name AS solver_name
    FROM Solve
    LEFT JOIN Puzzle ON Solve.puzzle_id = Puzzle.id -- must use LEFT JOIN to get join elimination
    LEFT JOIN Variant ON Solve.variant_id = Variant.id
    LEFT JOIN Program ON Solve.program_id = Program.id
    LEFT JOIN UserAccount ON Solve.solver_id = UserAccount.id;

CREATE OR REPLACE VIEW VerifiedFmcSolve AS
    SELECT * FROM InlinedSolve
    WHERE fmc_verified IS TRUE AND move_count IS NOT NULL;

CREATE OR REPLACE VIEW VerifiedSpeedSolve AS
    SELECT * FROM InlinedSolve
    WHERE speed_verified IS TRUE AND speed_cs IS NOT NULL;

CREATE OR REPLACE VIEW VerifiedSolve AS
    SELECT * FROM InlinedSolve
    WHERE fmc_verified IS TRUE OR speed_verified IS TRUE;

CREATE OR REPLACE VIEW VerifiedSpeedSolveInPrimaryCategory AS
    SELECT * FROM VerifiedSpeedSolve
    WHERE filters <= puzzle_primary_filters AND macros <= puzzle_primary_macros;

CREATE OR REPLACE VIEW PuzzleInfo AS
    SELECT
        id,
        (EXISTS (SELECT
            FROM VerifiedSolve
            WHERE VerifiedSolve.puzzle_id = Puzzle.id
                AND VerifiedSolve.average
        )) as has_average,
        (EXISTS (SELECT
            FROM VerifiedSolve
            WHERE VerifiedSolve.puzzle_id = Puzzle.id
                AND VerifiedSolve.program_id IS NULL
        )) as has_virtual,
        (EXISTS (SELECT
            FROM VerifiedSolve
            WHERE VerifiedSolve.puzzle_id = Puzzle.id
                AND VerifiedSolve.program_id IS NOT NULL
        )) as has_material
    FROM Puzzle;
