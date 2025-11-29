-- Add up migration script here

-- Mark non-FMC solves as not FMC-verified
UPDATE Solve SET fmc_verified_by = NULL, fmc_verified = NULL WHERE move_count IS NULL;
-- Mark non-speed solves as not speed-verified
UPDATE Solve SET speed_verified_by = NULL, speed_verified = NULL WHERE speed_cs IS NULL;

-- Mark verified solves without user as verified by migration
UPDATE Solve SET fmc_verified_by = 2 WHERE fmc_verified IS NOT NULL AND fmc_verified_by IS NULL;
UPDATE Solve SET speed_verified_by = 2 WHERE speed_verified IS NOT NULL AND speed_verified_by IS NULL;

-- Add SolveLog
CREATE TABLE IF NOT EXISTS SolveLog (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,

    editor_id INTEGER REFERENCES UserAccount NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    solve_id INTEGER REFERENCES Solve NOT NULL,
    description TEXT,

    visible_to_public BOOLEAN NOT NULL,
    visible_to_solver BOOLEAN NOT NULL
);

INSERT INTO SolveLog (editor_id, solve_id, description, visible_to_public, visible_to_solver)
SELECT
    2, Solve.id, 'Migrated from old schema', TRUE, TRUE
FROM Solve;

INSERT INTO SolveLog (editor_id, solve_id, description, visible_to_public, visible_to_solver)
SELECT
    2, Solve.id, CONCAT('FMC verified by ', UserAccount.name), TRUE, TRUE
FROM Solve
    LEFT JOIN UserAccount ON Solve.fmc_verified_by = UserAccount.id
    WHERE Solve.fmc_verified_by IS NOT NULL;

INSERT INTO SolveLog (editor_id, solve_id, description, visible_to_public, visible_to_solver)
SELECT
    2, Solve.id, CONCAT('Speed verified by ', UserAccount.name), TRUE, TRUE
FROM Solve
    LEFT JOIN UserAccount ON Solve.speed_verified_by = UserAccount.id
    WHERE Solve.speed_verified_by IS NOT NULL;

-- Add UserLog
CREATE TABLE IF NOT EXISTS UserLog (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,

    editor_id INTEGER REFERENCES UserAccount NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    user_id INTEGER REFERENCES UserAccount NOT NULL,
    description TEXT
);

INSERT INTO UserLog (editor_id, user_id, description)
SELECT
    2, id, 'Migrated from old schema'
FROM UserAccount;

-- Add GeneralLog
CREATE TABLE IF NOT EXISTS GeneralLog (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,

    editor_id INTEGER REFERENCES UserAccount NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    description TEXT
);

INSERT INTO GeneralLog (editor_id, description)
VALUES (2, 'Began audit logs');
