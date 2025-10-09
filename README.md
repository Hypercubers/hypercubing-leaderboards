# hypercubing-leaderboards

Combined database + API + web server + Discord bot for the [Hypercubing leaderboards](https://lb.hypercubing.xyz/)

## Setup

See [setup.md](setup.md)

## Instructions for moderators

Raed the [Panic mode](#panic-mode) and [Maintenance](#maintenance) sections. Don't worry about the things that require shell access.

Read the [Editing solve submissions](#editing-solve-submissions) and [Verifying solves](#verifying-solves) sections.

:name_badge: When rejecting a solve, edit the moderator notes and explain why it was rejected, along with your name and the date. For example:

```
[Hactar 2025-10-08] rejected because no keybinds reference
```

Also mention in the #leaderboard-submissions channel, and consider contacting the user to let them know.

## Usage

:name_badge: indicates functionality only available to moderators.

This is not a complete list of functionality, but it does include all moderator capabilities.

**Be extremely careful when editing the database directly.** Avoid doing this whenever possible. Use `BEGIN; ... COMMIT;` transaction block for safety.

### Panic mode

- :name_badge: `/panic block <thing>` disables various non-read functionality. Use this in case of an ongoing spam incident.
  - :name_badge: `/panic block logins` if someone is spamming new accounts.
  - :name_badge: `/panic block submissions` if someone is spamming submissions.
  - :name_badge: `/panic block user` if someone is spamming other things.
  - :name_badge: `/panic block moderator` if someone has moderator access that should not have it (although you should probably `/shutdown` at that point).
  - :name_badge: `/panic block all` if you don't know what's happening and you want to play it safe.
- :name_badge: `/panic unblock` re-enables all functionality.
- :name_badge: `/panic logout <target_user_id>` logs out a specific user everywhere.
- :name_badge: `/panic logout_all` logs out all users everywhere.

### Maintenance

- :name_badge: `/version` displays the Git commit hash, which can be compared against the latest one on GitHub.
- :name_badge: `/update` updates from the latest GitHub Actions build and restarts.
  - :floppy_disk: Whenever the server auto-updates, the old version is saved and can be restored with SSH access.
- :name_badge: `/restart` restarts the server.
- :name_badge: `/shutdown` shuts down the leaderboards bot and website. **This is irreversible without SSH access. Use this only as a last resort.**
- :computer: Restarting the leaderboard after shutting it down requires shell access: `~/hypercubing-leaderboards/hypercubing-leaderboards & disown`
- :computer: Updating the leaderboards to a new version with a database migration requires shell access: `~/hypercubing-leaderboards/hypercubing-leaderboards migrate`
- :floppy_disk: Backups are taken daily. Old backups are deleted exponentially.
  - All backups are kept for the last week.
  - One backup per day is kept for the last month.
  - One backup per month is kept for the last year.
  - One backup per year is kept for eternity.

### User Accounts

User IDs can be found in the URL for a user. For example, <https://lb.hypercubing.xyz/solver?id=8> is the URL for Hactar, so Hactar's user ID is 8.

#### Website

- Users can sign in using email address or Discord account.
- A new account is automatically created when someone signs in.
- Signing in sends a one-time code to the email address or Discord account.
- There are no passwords.
- Users can change their own display name, and are required to set one before submitting a solve.

#### Discord

- `/user show <target_user_id>` to show user info.
  - Contact details are visible only to moderators and the target user themself.
  - Moderator notes are visible only to moderators.
- :name_badge: `/user set name <target_user_id> <new_name>` to set a user's display name.
- :name_badge: `/user set discord <target_user_id> <new_discord_account>` to set a user's Discord account.
- :name_badge: `/user set email <target_user_id> <new_email>` to set a user's email address.
- :name_badge: `/user promote <target_user_id>` to promote a user to a leaderboard moderator.
- :name_badge: `/user demote <target_user_id>` to demote a leaderboard moderator to an ordinary user.

Moderators are allowed to email or DM Discord users as needed for verification reasons. At the time of writing, the email address <support@hypercubing.xyz> forwards to Hactar's personal email.

### Submitting solves

- Solves can be submitted [on the website](https://lb.hypercubing.xyz/submit-solve).

### Editing solve submissions

- :name_badge: On the main page, there is a link to all submissions awaiting verification.
- :name_badge: On a user's page, there is a link to all the submissions from a user.
- :name_badge: Moderators can edit any submission.
  - :name_badge: All properties of a solve (except for submission time) can be edited. This includes the solver, category, solve date, notes, etc.
- Users can view [all their submissions](https://lb.hypercubing.xyz/my-submissions)
- Users can edit their own unverified submissions.

### Verifying solves

Solve IDs can be found in the URL for a solve. For example, <https://lb.hypercubing.xyz/solve?id=224> is the link to the solve with ID 224.

#### Website

- :name_badge: Solves can be verified from their page on the website.
  - :name_badge: Speed and FMC are verified separately, even on the same solve.

#### Discord

- :name_badge: `/accept speed <solve_id>` to accept a speed submission
- :name_badge: `/accept fmc <solve_id>` to accept an FMC submission
- :name_badge: `/reject speed <solve_id>` to reject a speed submission
- :name_badge: `/reject fmc <solve_id>` to reject an FMC submission
- :name_badge: `/unverify speed <solve_id>` to unverify a speed submission
- :name_badge: `/unverify fmc <solve_id>` to unverify an FMC submission

### Adding new variants/programs/puzzles

- :name_badge: On the main page, there is a link to the [Categories](https://lb.hypercubing.xyz/categories) page, where modertors can edit or add new variants, programs, and puzzles.
- :computer: Deleting a variant, program, or puzzle requires shell access.

#### Limitations

- Users cannot currently change their own email or Discord account; this requires moderator intervention.

## Documentation

See [`docs/schema.md`](docs/schema.md) for an explanation of the database schema.

## License

This project is licensed under [MIT](https://opensource.org/license/mit) OR [Apache v2.0](https://apache.org/licenses/LICENSE-2.0).
