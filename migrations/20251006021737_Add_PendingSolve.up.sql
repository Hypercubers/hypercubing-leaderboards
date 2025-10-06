-- Add up migration script here
CREATE OR REPLACE VIEW PendingSolve AS
    SELECT *
    FROM InlinedSolve
    WHERE (speed_cs > 0 AND speed_verified IS NULL)
        OR (move_count > 0 AND fmc_verified IS NULL)
        OR (speed_verified IS NULL AND fmc_verified IS NULL)
    ORDER BY upload_date DESC;
