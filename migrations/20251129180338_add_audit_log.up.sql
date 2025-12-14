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
    json_data JSONB NOT NULL
);

-- Add "migrated" log entries
INSERT INTO SolveLog (editor_id, solve_id, json_data)
SELECT
    2, Solve.id, jsonb_build_object('type', 'migrated')
FROM Solve;

-- Add FMC verified status to "migrated" log entries
UPDATE SolveLog
    SET json_data = jsonb_set(json_data, '{fmc_verified}', jsonb_build_array(Solve.fmc_verified, UserAccount.id, UserAccount.name))
FROM Solve
    LEFT JOIN UserAccount ON Solve.fmc_verified_by = UserAccount.id
WHERE
    SolveLog.solve_id = Solve.id
    AND Solve.fmc_verified_by IS NOT NULL;

-- Add speed verified status to "migrated" log entries
UPDATE SolveLog
    SET json_data = jsonb_set(json_data, '{speed_verified}', jsonb_build_array(Solve.speed_verified, UserAccount.id, UserAccount.name))
FROM Solve
    LEFT JOIN UserAccount ON Solve.speed_verified_by = UserAccount.id
WHERE
    SolveLog.solve_id = Solve.id
    AND Solve.speed_verified_by IS NOT NULL;

-- Add UserLog
CREATE TABLE IF NOT EXISTS UserLog (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,

    editor_id INTEGER REFERENCES UserAccount NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    user_id INTEGER REFERENCES UserAccount NOT NULL,
    json_data JSONB NOT NULL
);

INSERT INTO UserLog (editor_id, user_id, json_data)
SELECT
    2, id, jsonb_build_object('type', 'migrated')
FROM UserAccount;

-- Add GeneralLog
CREATE TABLE IF NOT EXISTS GeneralLog (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,

    editor_id INTEGER REFERENCES UserAccount NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    json_data JSONB NOT NULL
);

INSERT INTO GeneralLog (editor_id, json_data)
VALUES (2, jsonb_build_object('type', 'started'));
