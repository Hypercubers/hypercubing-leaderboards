# Schema

## Tables

### UserAccount

- `id: integer primary key`
- `name: optional varchar(255)` — display name, can be changed by the user
- `moderator_notes: text`
- Contact info
  - `email: optional varchar(255)`
  - `discord_id: optional bigint`
- Flags
  - `moderator: boolean`
  - `dummy: boolean`

### Token

Used for authentication. Generated automatically when a user logs into the website or Hyperspeedcube.

- `id: integer primary key`
- `user_id: UserAccount.id`
- `token: char(64)`
- `expiry: timestamp` — when the token will expire

### Program

Software used for hypercubing, or the special "N/A" program used for material solves.

- `id: integer primary key`
- `name: varchar(255)` — user-friendly name of the program (e.g., `Hyperspeedcube 2`)
- `abbr: varchar(255)` — user-friendly and URL-safe abbreviation of the program name (e.g., `HSC2`)
- `material: boolean` — whether this is the special "N/A" program used for material solves

### Variant

Puzzle variant, used for special categories. At time of writing, there are only two variants: "Physical" (which is material) and "1D Vision" (which is virtual).

- `id: integer primary key`
- `name: varchar(255)` — user-friendly name of the variant (e.g., `Physical`)
- `prefix: varchar(255)` — prefix to append to a puzzle name (e.g., `Physical `; note the trailing space)
- `suffix: varchar(255)` — suffix to append to a puzzle name (e.g., : with` 1D Vision`; note the leading space)
- `abbr: varchar(255)` — user-friendly and URL-safe abbreviation of the variant name (e.g., `phys`)
- `material_by_default: boolean` — whether this variant is primarily material (used to determine whether to append a `Virtual` or `Material` prefix when displaying the other category)
- `primary_filters: boolean` — whether the main category for this variant allows piece filters (takes precedence over puzzle `primary_filters`)
- `primary_macros: boolean` — whether the main category for this variant allows piece filters (takes precedence over puzzle `primary_macros`)

### Puzzle

- `id: integer primary key`
- `name: varchar(255)`
- `primary_filters: boolean` — whether the main category for the puzzle allows piece filters
- `primary_macros: boolean` — whether the main category for the puzzle allows macros

### HscPuzzle

Connection between a puzzle in Hyperspeedcube and a puzzle on the leaderboard. Each HSC puzzle links to at most one leaderboard puzzle, but each leaderboard puzzle may link to many HSC puzzles.

- `hsc_id: varchar(255) primary key` — ID in Hyperspeedcube (e.g., `ft_cube:3`)
- `puzzle_id: Puzzle.id`

### Solve

A solve must contain either `move_count` or `speed_cs`, or both. If a solve contains non-null `move_count`, it is an FMC (fewest-move-count) submission. If it contains `speed_cs`, it is a speedsolve submission. The `move_count` and `speed_cs` can each be independently verified (`verified=true`) or rejected (`verified=false`).

- `id: integer primary key`
- Metadata
  - `solver_id: UserAccount.id`
  - `solve_date: timestamp`
  - `upload_date: timestamp`
  - `solver_notes: text`
  - `moderator_notes: text`
- Event
  - `puzzle_id: Puzzle.id`
  - `variant_id: optional Variant.id` — (for speedsolves)
  - `program_id: Program.id`
  - `average: boolean` — (for speedsolves) whether this is an Ao5 submission (speedsolves only)
  - `blind: boolean` — (for speedsolves) whether this is a blindsolve
  - `filters: boolean` — (for speedsolves) whether this solve used filters
  - `macros: boolean` — (for speedsolves) whether this solve used macros
  - `one_handed: boolean` — (for speedsolves) whether this solve was one-handed
  - `computer_assisted: boolean` — (for FMC solves) whether this solve used computer assistance in generating the solution
- Score
  - `move_count: optional integer` — (for FMC solves) number of [STM](https://hypercubing.xyz/notation/#turn-metrics) twists in the solution
  - `speed_cs: optional integer` — (for speedsolves) number of centiseconds for the full solve
  - `memo_cs: optional integer` — (for blindsolves) number of centiseconds used for memorization
- Verification
  - `fmc_verified: optional boolean` — `NULL` if not verified, `true` if accepted, `false` if rejected
  - `fmc_verified_by: UserAccount.id` — moderator that verified the solve
  - `speed_verified: optional boolean` — `NULL` if not verified, `true` if accepted, `false` if rejected
  - `speed_verified_by: UserAccount.id` — moderator that verified the solve
- Evidence
  - `log_file_name: optional TEXT`
  - `log_file_contents: optional BYTEA`
  - `scramble_seed: optional CHAR(64)`
  - `video_url: optional TEXT`

The following category inclusions apply:

- Solves with `filters: false` count for filters categories
- Solves with `macros: false` count for macros categories
- Solves with `one_handed: true` count for non-one-handed categories
- Solves with `computer_assisted: false` count for computer-assisted categories

Solves with `computer_assisted: true` are disqualified from speedsolves.

`memo_cs` is only valid for blindsolves, but is never required.
