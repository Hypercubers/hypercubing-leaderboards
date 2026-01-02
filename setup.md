# Leaderboards Server Setup

## Development

1. Install [Rust](https://www.rust-lang.org/tools/install)
2. Clone this repository:

```sh
git clone https://github.com/Hypercubers/hypercubing-leaderboards.git
cd hypercubing-leaderboards
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

9. Copy [`.env.example`](.env.example) to `.env` with `cp .env.example .env`
10. Follow the [Database setup](#database-setup) and [Discord bot setup](#discord-bot-setup) instructions
10. If you want to test email functionality, follow the [Email setup](#email-setup) instructions
11. If you want to test automatic solve verification using HSC2, ensure that the value for `HSC2_PATH` in `.env` is a path to a `hyperspeedcube` executable

## Deployment (Linux)

I recommend deploying the leaderboards server on the same machine that is running the Hypercubing Nextcloud. See [Server Setup on dev.hypercubing.xyz](https://dev.hypercubing.xyz/infrastructure/server/).

1. Install the [GitHub CLI](https://cli.github.com/) with `sudo apt install gh`
2. [Create a personal access token](https://github.com/settings/tokens), preferably with the Hypercubers organization as the resource owner and a long expiration.

  - Set the **resource owner** to the Hypercubers organization
  - Set the **expiration** to 1 year
  - Set the **repository access** to **Public repositories (read-only)**

3. Run `gh run download --repo hypercubers/hypercubing-leaderboards --name linux`
4. Extract the file with `unzip linux.zip`
5. Copy [`.env.example`](.env.example) to `.env` with `cp .env.example .env`
6. Follow the [Database setup](#database-setup) and [Discord bot setup](#discord-bot-setup) instructions
7. Intialize the database from `solves.csv` with `./hypercubing-leaderboards init`

You can use `psql -U leaderboards_bot -h 127.0.0.1 leaderboards` to access the database directly, although it's best to avoid this. Remember to always `BEGIN TRANSACTION` before making any changes!

## Database setup

1. Install [PostgreSQL](https://www.postgresql.org/download/)

  - On macOS: `brew install postgresql@16`
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

3. Ensure that the value for `DATABASE_URL` in `.env` matches what you used in step 2:

  - If necessary, replace `leaderboards_bot` with the user you created
  - If necessary, replace `password` with the password you created
  - If necessary, replace `leaderboards` with the database you created

The password does not need to be secure because Postgres is not exposed to public internet

## Discord bot setup

If you already have a bot, reset its token and skip to step 4. **Do not use the same bot for testing and production.**

1. Go to the [Discord Developer Portal](https://discord.com/developers/applications)
2. Create a new application
3. Create a bot user for your application
4. Fill in  `.env` as follows:

  - Set `DISCORD_TOKEN` equal to your Discord bot token
  - Set `VERIFICATION_CHANNEL_ID` equal to the ID of a channel that is only visible to moderators (this should be a number with ~19 digits)
  - Set `UPDATE_CHANNEL_ID` equal to the ID of a channel that is only visible to moderators (this should be a number with ~19 digits)

To get a channel's ID, go to your Discord user settings under **Advanced** and enable **Developer Mode**, then right-click on a channel and click **Copy Channel ID**.

## Email setup

1. Create a free account with [Mailtrap](https://mailtrap.io/) or some other SMTP provider
2. Go to [Sending Domains](https://mailtrap.io/sending/domains) and add `hypercubing.xyz`
3. Follow the "Domain Verification" instructions
4. Set `SMTP_HOST`, `SMTP_HOST_PORT`, `SMTP_USERNAME`, and `SMTP_PASSWORD` accordingly in `.env`
5. Ensure that `support@hypercubing.xyz` forwards to your personal email address or some other address that you will see. You can configure

## Cloudflare Turnstile setup

We use Cloudflare [Turnstile](https://www.cloudflare.com/application-services/products/turnstile/) to prevent bots from creating accounts or spamming people with email.

1. Add a Turnstile widget in Cloudflare
2. Set `TURNSTILE_SITE_KEY` and `TURNSTILE_SECRET_KEY` accordingly in `.env`

For testing locally, use the following values:

```sh
TURNSTILE_SITE_KEY=1x00000000000000000000AA
TURNSTILE_SECRET_KEY=1x0000000000000000000000000000000AA
```

or see [Testing · Cloudflare Turnstile docs](https://developers.cloudflare.com/turnstile/troubleshooting/testing/) for more options.

## Startup

To run the leaderboards on startup on Linux, you can create a cron job by running `crontab -e` and add the following line:

```cron
@reboot sh -c "cd ~/hypercubing-leaderboards/ && ./hypercubing-leaderboards"
```

## Daily backups

To back up the database to another server:

1. On the **backup** server, [Generate an SSH key](https://docs.github.com/en/authentication/connecting-to-github-with-ssh/generating-a-new-ssh-key-and-adding-it-to-the-ssh-agent) if you do not already have one.
2. On the **leaderboards** server, authorize your public key for the `postgres` user:

```sh
sudo mkdir -p /var/lib/postgresql/.ssh
sudo nano /var/lib/postgresql/.ssh/authorized_keys
# add your public key and then save the file
sudo nano /var/lib/postgresql/.pgpass
# add the following line and then save the file:
# 127.0.0.1:5432:leaderboards:leaderboards_bot:password
sudo chown postgres /var/lib/postgresql/.pgpass
sudo chmod 600 /var/lib/postgresql/.pgpass
```

3. On the **backup** server, verify that you can SSH into the leaderboards server:

```sh
ssh postgres@lb.hypercubing.xyz echo success!
```

3. On the **backup** server, download [`backup_leaderboards_db.py`](backup_leaderboards_db.py) to some directory (e.g., `~/scripts`):

```sh
curl https://raw.githubusercontent.com/Hypercubers/hypercubing-leaderboards/refs/heads/main/backup_leaderboards_db.py > ~/scripts/backup_leaderboards_db.py
```

4. On the **backup** server, make a directory for the backups (e.g., `~/hypercubing_leaderboards_db_backups`).
5. On the **backup** server, run `crontab -e` and add the following line, using the paths from step 3 and 4:

```cron
@daily python3 ~/scripts/backup_leaderboards_db.py -r postgres@lb.hypercubing.xyz ~/hypercubing_leaderboards_db_backups
```

Note that this will only run if the backup server is online at midnight, so this is mostly only useful on a device that is generally running 24/7.

## Useful scripts

Remember to run `chmod +x whatever-file-name.sh` to mark these as executable.

- `update.sh` and `update-hsc.sh` are required for the `/update` Discord bot command.

### `psql.sh`

```bash
#!/bin/bash
psql -U leaderboards_bot -h 127.0.0.1 leaderboards "$@"
```

### `restart-cron.sh`

```sh
#!/bin/sh
sudo rm /var/run/crond.reboot
sudo systemctl restart cron.service
```

### `update.sh`

```sh title="update.sh"
#!/bin/bash
set -e
mv hypercubing-leaderboards hypercubing-leaderboards.old."$(date +'%Y-%m-%d.%H-%M-%S')"
gh run download --repo hypercubers/hypercubing-leaderboards --name linux
```

### `update-hsc.sh`

```sh title="update-hsc.sh"
#!/bin/bash
set -e
mv hyperspeedcube hyperspeedcube.old."$(date +'%Y-%m-%d.%H-%M-%S')"
gh run download --repo HactarCE/Hyperspeedcube --name hyperspeedcube_linux
tar -xf hyperspeedcube_linux.tar.gz
rm hyperspeedcube_linux.tar.gz
```
