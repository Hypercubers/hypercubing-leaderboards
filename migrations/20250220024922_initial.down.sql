DROP VIEW IF EXISTS
    PuzzleInfo,
    VerifiedSpeedSolveInPrimaryVariant,
    VerifiedSolve,
    VerifiedSpeedSolve,
    VerifiedFmcSolve,
    InlinedSolve
    CASCADE;

DROP TABLE IF EXISTS
    Solve,
    SpecialEvent,
    HscPuzzle,
    Puzzle,
    Variant,
    Program,
    Token,
    UserAccount
    CASCADE;
