# hsc-leaderboard

Combined database + API + web server + Discord bot for the [Hypercubing leaderboards](https://lb.hypercubing.xyz/)

## Setup

### Development

1. Install [Rust](https://www.rust-lang.org/tools/install)
2. Clone this repository:

```sh
git clone https://github.com/Hypercubers/hsc-leaderboard.git
cd hsc-leaderboard
```

3. Install `sqlx` with `cargo install sqlx-cli`

  - On Ubuntu, you may need to run `sudo apt install gcc libssl-dev pkg-config` first

4. Follow the [database setup instructions](#database-setup)
5. Run `cargo build`

Before committing, run `cargo sqlx prepare`. To do this automatically on every commit, create a pre-commit hook with this command:

```sh
echo "cargo sqlx prepare > /dev/null 2>&1; git add .sqlx > /dev/nu
ll" > .git/hooks/pre-commit && chmod +x .git/hooks/pre-commit
```

### Running

1. Install the [GitHub CLI](https://cli.github.com/)

  - On macOS: `brew install gh`
  - On Ubuntu: `sudo apt install gh`

2. Run `gh run download --repo hypercubers/hsc-leaderboard` and select the latest version
3. Follow the [database setup instructions](#database-setup)

### Database setup

1. Install [PostgreSQL](https://www.postgresql.org/download/)

  - On macOS: `brew install postgresql`
  - On Ubuntu: `sudo apt install postgresql && sudo systemctl enable --now postgresql`

2. Run `psql -d template1`. Using that prompt, set up the database:

```sql
CREATE DATABASE leaderboards;
CREATE USER leaderboards_bot WITH PASSWORD 'password';
ALTER DATABASE leaderboards OWNER TO leaderboards_bot;
exit;
```

3. Create a `.env` file based on [`.env.example`](.env.example)

  - (Optional) Replace `leaderboards_bot` with the database user you used in step 2
  - (Optional) Replace `leaderboards` with the database name you used in step 2
  - (Optional) Replace `password` with the database password you used in step 2
  - Replace `YOUR_DISCORD_TOKEN_HERE` with your Discord bot token

4. Initialize the database with `cargo sqlx migrate run`

## License

This project is licensed under [MIT](https://opensource.org/license/mit) OR [Apache v2.0](https://apache.org/licenses/LICENSE-2.0).
