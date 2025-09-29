#!/usr/bin/env python3

import os
import time
from datetime import datetime, timedelta
from os import path
import subprocess
import sys

DRY_RUN = False

def print_help(exit_code):
    this_file = sys.argv[0]
    print("Usage:")
    print(f"    python3 {this_file} --help")
    print(f"    python3 {this_file} <database_name> <backup_directory> [--dry-run]")
    sys.exit(exit_code)

if '-h' in sys.argv[1:] or '--help' in sys.argv[1:]:
    print_help(0)

try:
    [_, DATABASE_NAME, BACKUP_DIR, *args] = sys.argv
    for arg in args:
        match arg:
            case '--dry-run':
                DRY_RUN = True
            case _:
                print("Unknown flag {arg!r}")
                print_help(1)
except:
    print_help(1)

FILENAME_FORMAT = f'{DATABASE_NAME}.%Y-%m-%d.%H-%M-%S.sql'

if DRY_RUN:
    print("THIS IS A DRY RUN; NO ACTUAL FILES WILL BE CREATED OR DELETED")
    print()

os.makedirs(BACKUP_DIR, exist_ok=True)

# Save new backup
now = datetime.now()
output_file = path.join(BACKUP_DIR, now.strftime(FILENAME_FORMAT))
print(f"Backing up database to {output_file} ...")
if not DRY_RUN:
    with open(output_file, 'wb') as f:
        f.write(subprocess.run(['pg_dump', DATABASE_NAME], capture_output=True).stdout)
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
        os.remove(path.join(BACKUP_DIR, filename))

if DRY_RUN:
    print()
    print("THIS WAS A DRY RUN; NO ACTUAL FILES WERE CREATED OR DELETED")
