DROP TABLE IF EXISTS
    UserAccount,
    Token,
    Program,
    ProgramVersion,
    Puzzle,
    Solve,
    SpeedEvidence,
    DiscordConnection
    CASCADE;

DROP VIEW IF EXISTS
    LeaderboardSolve;

--DROP TRIGGER update_solve_rankings;
