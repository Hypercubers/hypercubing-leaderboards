#!/usr/bin/env python3

from datetime import datetime
import argparse
import gzip
import os
import subprocess
import sys

parser = argparse.ArgumentParser(
    prog='backup_db.py',
    description='Backs up all postgres databases and culls backups to a limited frequency',
)
parser.add_argument('backup_directory')
parser.add_argument('-u', '--user', help="system user to run as")
parser.add_argument('-d', '--dry-run', action='store_true', help="do not modify files")
args = parser.parse_args(sys.argv[1:])

BACKUP_DIR = args.backup_directory

DRY_RUN = args.dry_run

BACKUP_COMMAND = ['pg_dumpall']
if args.user:
    BACKUP_COMMAND = ['su', '-', args.user, '-c'] + BACKUP_COMMAND

def print_help(exit_code):
    this_file = sys.argv[0]
    print("Usage:")
    print(f"    python3 {this_file} --help")
    print(f"    python3 {this_file} <backup_directory> [] [--dry-run]")
    sys.exit(exit_code)

FILENAME_FORMAT = f'%Y-%m-%d.%H-%M-%S.gz'

if DRY_RUN:
    print("THIS IS A DRY RUN; NO ACTUAL FILES WILL BE CREATED OR DELETED")
    print()

os.makedirs(BACKUP_DIR, exist_ok=True)

# Save new backup
now = datetime.now()
output_file = os.path.join(BACKUP_DIR, now.strftime(FILENAME_FORMAT))
print(f"Backing up database to {output_file} ...")
if not DRY_RUN:
    with open(output_file, 'wb') as f:
        bytes = subprocess.run(BACKUP_COMMAND, capture_output=True).stdout
        compressed_bytes = gzip.compress(bytes)
        f.write(compressed_bytes)
print("Backup successful!")

print()

print("Culling backups ...")
years = set() # years where we have a backup
months = set() # (year, month) tuples in the last year where we have a backup
# Iterate in sorted order from oldest to newest
for filename in sorted(os.listdir(BACKUP_DIR)):
    try:
        backup_date = datetime.strptime(filename, FILENAME_FORMAT)
    except ValueError:
        print(f"Ignoring unknown file {filename}")
        continue

    year = backup_date.year
    month = backup_date.month
    age = now - backup_date

    if age.days < 0:
        print(f"Keeping {filename} (from the future)")
        continue
    if age.days < 7:
        # this is within the last week; keep it
        print(f"Keeping {filename} (within the last week)")
        continue
    elif age.days < 365:
        # this is within the last year; keep one per month
        key = (year, month)
        if key not in months:
            print(f"Keeping {filename} (oldest backup for {year:04}-{month:02})")
            months.add(key)
            continue
    else:
        # keep one per year
        key = year
        if key not in years:
            print(f"Keeping {filename} (oldest backup for {year:04})")
            years.add(key)
            continue

    print(f"Deleting backup {filename}")
    if not DRY_RUN:
        os.remove(os.path.join(BACKUP_DIR, filename))

if DRY_RUN:
    print()
    print("THIS WAS A DRY RUN; NO ACTUAL FILES WERE CREATED OR DELETED")
