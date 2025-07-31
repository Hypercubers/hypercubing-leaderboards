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

3. Follow the [database setup instructions](#database-setup)
4. Install `sqlx` with `cargo install sqlx-cli`

  - On Ubuntu, you may need to run `sudo apt install gcc libssl-dev pkg-config` first

5. Initialize the database schema with `cargo sqlx migrate run`
6. (optional) Create a file `discord_accounts.txt` containing lines of the form `name_in_solvers_yml DISCORD_USER_ID`
7. Run `cargo run -- reset && cargo run -- init` to reinitialize the database from `solves.csv`
8. Run `cargo run` to run the web server and Discord bot

Before committing, run `cargo sqlx prepare`. To do this automatically on every commit, create a pre-commit hook with this command:

```sh
echo "cargo sqlx prepare > /dev/null 2>&1; git add .sqlx > /dev/nu
ll" > .git/hooks/pre-commit && chmod +x .git/hooks/pre-commit
```

### Running (Linux)

1. Install the [GitHub CLI](https://cli.github.com/) with `sudo apt install gh`
2. [Create a personal access token](https://github.com/settings/tokens), preferably with the Hypercubers organization as the resource owner and a long expiration.

  - Set the **resource owner** to the Hypercubers organization
  - Set the **expiration** to 1 year
  - Set the **repository access** to **Public repositories (read-only)**

3. Run `gh run download --repo hypercubers/hsc-leaderboard --name linux`
4. Extract the file with `unzip linux.zip`
5. Follow the [database setup instructions](#database-setup)
6. Intialize the database from `solves.csv` with `./hsc-leaderboard init`

You can use `psql -U leaderboards_bot -h 127.0.0.1 leaderboards` to access the database directly, although it's best to avoid this. Remember to always `BEGIN TRANSACTION` before making any changes!

### Database setup

1. Install [PostgreSQL](https://www.postgresql.org/download/)

  - On macOS: `brew install postgresql`
  - On Ubuntu: `sudo apt install postgresql-client && sudo systemctl enable --now postgresql`

2. Access the PSQL prompt and use it to set up the database.

  - On macOS: `psql -d template1`
  - On Linux: `sudo -u postgres psql`

```sql
CREATE DATABASE leaderboards;
CREATE USER leaderboards_bot WITH PASSWORD 'password';
ALTER DATABASE leaderboards OWNER TO leaderboards_bot;
\q
```

3. Create a `.env` file based on [`.env.example`](.env.example)

  - (Optional) Replace `leaderboards_bot` with the database user you used in step 2
  - (Optional) Replace `leaderboards` with the database name you used in step 2
  - (Optional) Replace `password` with the database password you used in step 2
  - Replace `YOUR_DISCORD_TOKEN_HERE` with your Discord bot token

## Documentation

See [`docs/schema.md`](docs/schema.md) for an explanation of the database schema.

## License

This project is licensed under [MIT](https://opensource.org/license/mit) OR [Apache v2.0](https://apache.org/licenses/LICENSE-2.0).
