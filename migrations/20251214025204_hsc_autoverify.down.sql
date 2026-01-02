DROP VIEW IF EXISTS
    VerifiedFmcSolve,
    VerifiedSpeedSolve,
    VerifiedSolve,
    VerifiedSpeedSolveInPrimaryCategory,
    InlinedSolve,
    PendingSolve
    CASCADE;

ALTER TABLE Solve DROP COLUMN auto_verify_output;

ALTER TABLE Puzzle DROP COLUMN hsc_id;
ALTER TABLE Puzzle DROP COLUMN autoverifiable;

-- Recreate views

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

        -- Evidence (continued at bottom)
        Solve.log_file_name, -- log file may be too big
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

CREATE OR REPLACE VIEW PendingSolve AS
    SELECT *
    FROM InlinedSolve
    WHERE (speed_cs > 0 AND speed_verified IS NULL)
        OR (move_count > 0 AND fmc_verified IS NULL)
        OR (speed_verified IS NULL AND fmc_verified IS NULL)
    ORDER BY upload_date DESC;
