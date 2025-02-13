# hsc-leaderboard

Combined database + API + web server + Discord bot for the [Hypercubing leaderboards](https://lb.hypercubing.xyz/)

## Setup

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
exit
```

3. Create a `.env` file based on [`.env.example`](.env.example)

  - (Optional) Replace `leaderboards_bot` with the database user you used in step 2
  - (Optional) Replace `leaderboards` with the database name you used in step 2
  - (Optional) Replace `password` with the database password you used in step 2
  - Replace `YOUR_DISCORD_TOKEN_HERE` with your Discord bot token

### Development

1. Follow the [database setup instructions](#database-setup)
2. Install [Rust](https://www.rust-lang.org/tools/install)
3. Clone this repository:

```sh
git clone https://github.com/Hypercubers/hsc-leaderboard.git
cd hsc-leaderboard
```

4. Install `sqlx` with `cargo install sqlx-cli`

  - On Ubuntu, you may need to run `sudo apt install gcc libssl-dev pkg-config` first

5. Initialize the database with `cargo sqlx migrate run`
6. Run `cargo build`

Before committing, run `cargo sqlx prepare`. To do this automatically on every commit, create a pre-commit hook with this command:

```sh
echo "cargo sqlx prepare > /dev/null 2>&1; git add .sqlx > /dev/nu
ll" > .git/hooks/pre-commit && chmod +x .git/hooks/pre-commit
```

### Running (Linux)

1. Follow the [database setup instructions](#database-setup)
2. Intialize the database by downloading the latest [migration file](migrations) to `migrate.sql` and running `psql -U leaderboards_bot -h 127.0.0.1 leaderboards -c '\ir migrate.sql'`
3. Install the [GitHub CLI](https://cli.github.com/) with `sudo apt install gh`
4. [Create a personal access token](https://github.com/settings/tokens), preferably with the Hypercubers organization as the resource owner and a long expiration.

    - Set the **resource owner** to the Hypercubers organization
    - Set the **expiration** to 1 year
    - Set the **repository access** to **Public repositories (read-only)**

6. Run `gh run download --repo hypercubers/hsc-leaderboard --name linux`
7. Extract the file with `unzip linux.zip`

## License

This project is licensed under [MIT](https://opensource.org/license/mit) OR [Apache v2.0](https://apache.org/licenses/LICENSE-2.0).
