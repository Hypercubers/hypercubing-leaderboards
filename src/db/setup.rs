use std::{
    collections::{hash_map, HashMap},
    error::Error,
};

use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use eyre::{bail, ensure, eyre, Result, WrapErr};
use itertools::Itertools;
use sqlx::{query, query_as};

use crate::AppState;

impl AppState {
    pub async fn reset(&self) -> Result<()> {
        println!(
            "Enter 'reset' (lowercase, without quotes) below to reset the database completely."
        );
        println!("Enter anything else to exit the program.");
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf)?;
        if buf == "reset" {
            println!("Resetting database ...");
            let mut transaction = self.pool.begin().await?;
            query!("DROP SCHEMA public CASCADE")
                .execute(&mut *transaction)
                .await?;
            query!("CREATE SCHEMA public")
                .execute(&mut *transaction)
                .await?;
            let _ = query!("GRANT ALL ON SCHEMA public TO postgres")
                .execute(&mut *transaction)
                .await; // ok if this fails
            query!("GRANT ALL ON SCHEMA public TO public")
                .execute(&mut *transaction)
                .await?;
            Ok(())
        } else {
            println!("Canceled");
            Ok(())
        }
    }

    pub async fn migrate(&self) -> Result<()> {
        sqlx::migrate!().run(&self.pool).await?;
        Ok(())
    }

    pub async fn init_puzzles(&self) -> Result<()> {
        let mut transaction = self.pool.begin().await?;

        query!("DELETE FROM Variant CASCADE")
            .execute(&mut *transaction)
            .await?;
        query!(
            "INSERT INTO Variant (name, prefix, suffix, abbr, material_by_default) VALUES
                ('Physical', 'Physical ', '', 'phys', TRUE),
                ('1D Vision', '', ' with 1D Vision', '1d', FALSE)
            "
        )
        .execute(&mut *transaction)
        .await?;

        query!("DELETE FROM Puzzle CASCADE")
            .execute(&mut *transaction)
            .await?;
        query!(
            "INSERT INTO Puzzle (name, primary_filters, primary_macros) VALUES
                ('3×3×3×3', TRUE, FALSE),
                ('2×2×2×2', TRUE, FALSE),
                ('4×4×4×4', TRUE, FALSE),
                ('5×5×5×5', TRUE, FALSE),
                ('6×6×6×6', TRUE, FALSE),
                ('7×7×7×7', TRUE, FALSE),
                ('1×3×3×3', TRUE, FALSE),
                ('2×2×2×3', TRUE, FALSE),
                ('2×2×3×3', TRUE, FALSE),
                ('3-Layer Simplex', TRUE, FALSE),
                ('3×3×3×3×3', TRUE, FALSE),
                ('2×2×2×2×2', TRUE, FALSE),
                ('4×4×4×4×4', TRUE, FALSE),
                ('Hemimegaminx', TRUE, FALSE),
                ('Canon-Cut Klein Quartic', TRUE, FALSE),
                ('3×3×3', TRUE, FALSE)
            "
        )
        .execute(&mut *transaction)
        .await?;

        query!("DELETE FROM HscPuzzle CASCADE")
            .execute(&mut *transaction)
            .await?;
        for (hsc_id, puzzle_name) in [
            ("ft_hypercube:3", "3×3×3×3"),
            ("ft_hypercube:2", "2×2×2×2"),
            ("ft_hypercube:4", "4×4×4×4"),
            ("ft_hypercube:5", "5×5×5×5"),
            ("ft_hypercube:6", "6×6×6×6"),
            ("ft_hypercube:7", "7×7×7×7"),
            // ("unknown", "1×3×3×3"),
            // ("unknown", "2×2×2×3"),
            // ("unknown", "2×2×3×3"),
            // ("unknown", "3-Layer Simplex"),
            ("ft_5_cube:3", "3×3×3×3×3"),
            ("ft_5_cube:2", "2×2×2×2×2"),
            ("ft_5_cube:4", "4×4×4×4×4"),
            // ("unknown", "Hemimegaminx"),
            // ("unknown", "Canon-Cut Klein Quartic"),
            ("3×3×3", "ft_cube:3"),
        ] {
            query!(
                "INSERT INTO HscPuzzle (hsc_id, puzzle_id)
                    SELECT $1, Puzzle.id
                    FROM Puzzle WHERE Puzzle.name = $2
                ",
                hsc_id,
                puzzle_name,
            )
            .execute(&mut *transaction)
            .await?;
        }

        transaction.commit().await?;

        Ok(())
    }

    pub async fn init_solves(&self) -> Result<()> {
        let mut transaction = self.pool.begin().await?;

        // Reset tables
        query!("DELETE FROM UserAccount CASCADE")
            .execute(&mut *transaction)
            .await?;
        query!("DELETE FROM Program CASCADE")
            .execute(&mut *transaction)
            .await?;
        query!("DELETE FROM Solve CASCADE")
            .execute(&mut *transaction)
            .await?;

        // Add programs
        let mut program_ids: HashMap<String, i32> = query!(
            "
            INSERT INTO Program (abbr, name, material) VALUES
                ('-', 'N/A', TRUE),
                ('AKS', 'Akkei’s physical 3^4 program', FALSE),
                ('FHC', 'Flat Hypercube', FALSE),
                ('HSC1', 'Hyperspeedcube 1', FALSE),
                ('HSC2', 'Hyperspeedcube 2', FALSE),
                ('LAM', 'Laminated', FALSE),
                ('MC3D', 'Magic Cube 3D', FALSE),
                ('MC4D', 'Magic Cube 4D', FALSE),
                ('MC5D', 'Magic Cube 5D', FALSE),
                ('MC7D', 'Magic Cube 7D', FALSE),
                ('MPU', 'MagicPuzzleUltimate', FALSE),
                ('MT', 'MagicTile', FALSE),
                ('X', 'Other Virtual', FALSE)
                RETURNING abbr, id
            "
        )
        .fetch_all(&mut *transaction)
        .await?
        .into_iter()
        .map(|row| (row.abbr, row.id))
        .collect();
        program_ids.insert("AKKEI-SIM".to_string(), *program_ids.get("AKS").unwrap());
        program_ids.insert("HSC".to_string(), *program_ids.get("HSC1").unwrap());

        // Get Discord accounts, if available
        let discord_accounts_txt = std::fs::read_to_string("discord_accounts.txt");
        let solver_discord_accounts = match &discord_accounts_txt {
            Ok(contents) => contents
                .lines()
                .filter_map(|line| line.split_once(' '))
                .filter_map(|(k, v)| Some((k, v.parse::<i64>().ok()?)))
                .collect(),
            Err(_) => HashMap::new(),
        };

        let solvers_yml = reqwest::get(
            "https://raw.githubusercontent.com/Hypercubers/hypercubing.xyz/refs/heads/main/leaderboards/solvers.yml",
        )
        .await?
        .text()
        .await?;
        let solver_names: HashMap<&str, &str> = solvers_yml
            .lines()
            .filter_map(|line| line.split_once(": "))
            .collect();

        let solves_csv = reqwest::get(
            "https://raw.githubusercontent.com/Hypercubers/hypercubing.xyz/refs/heads/main/leaderboards/solves.csv",
        )
        .await?
        .text()
        .await?;

        let puzzle_ids: HashMap<String, i32> = query!("SELECT name, id FROM Puzzle")
            .map(|row| (row.name, row.id))
            .fetch_all(&mut *transaction)
            .await?
            .into_iter()
            .collect();
        let variant_ids: HashMap<String, i32> = query!("SELECT abbr, id FROM Variant")
            .map(|row| (row.abbr, row.id))
            .fetch_all(&mut *transaction)
            .await?
            .into_iter()
            .collect();
        let puzzle_old_id_to_puzzle_and_variant_id: HashMap<&str, (i32, Option<i32>)> = [
            ("3x3x3x3", ("3×3×3×3", None)),
            ("2x2x2x2", ("2×2×2×2", None)),
            ("4x4x4x4", ("4×4×4×4", None)),
            ("5x5x5x5", ("5×5×5×5", None)),
            ("6x6x6x6", ("6×6×6×6", None)),
            ("7x7x7x7", ("7×7×7×7", None)),
            ("1x3x3x3", ("1×3×3×3", None)),
            ("2x2x2x3", ("2×2×2×3", None)),
            ("2x2x3x3", ("2×2×3×3", None)),
            ("phys_2x2x2x2", ("2×2×2×2", Some("phys"))),
            ("phys_3x3x3x3", ("3×3×3×3", Some("phys"))),
            ("virt_phys_3x3x3x3", ("3×3×3×3", Some("phys"))),
            ("3-layer_simplex", ("3-Layer Simplex", None)),
            ("3x3x3x3x3", ("3×3×3×3×3", None)),
            ("2x2x2x2x2", ("2×2×2×2×2", None)),
            ("4x4x4x4x4", ("4×4×4×4×4", None)),
            ("hemimegaminx", ("Hemimegaminx", None)),
            ("klein_quartic", ("Canon-Cut Klein Quartic", None)),
            ("3x3x3_1d", ("3×3×3", Some("1d"))),
        ]
        .into_iter()
        .map(|(old_id, (puzzle_name, variant_name))| {
            let puzzle_id = *puzzle_ids
                .get(puzzle_name)
                .ok_or_else(|| eyre!("unknown puzzle {puzzle_name:?}"))?;
            let variant_id = match variant_name {
                Some(s) => Some(
                    *variant_ids
                        .get(s)
                        .ok_or_else(|| eyre!("unknown variant {s}"))?,
                ),
                None => None,
            };
            eyre::Ok((old_id, (puzzle_id, variant_id)))
        })
        .try_collect()?;

        // Add solves and solvers
        let mut solver_ids = HashMap::<&str, i32>::new();
        for line in solves_csv.lines().skip(1) {
            if line.is_empty() {
                continue;
            }

            let components = line.split(',').map(str::trim).collect_vec();

            let date = components[0]
                .parse::<NaiveDate>()?
                .and_time(NaiveTime::from_hms_opt(12, 0, 0).unwrap())
                .and_utc();

            let mut video_url = components[1].to_string();
            if !video_url.contains("://") {
                video_url = format!("https://youtu.be/{video_url}");
            }

            let speed_cs = components[2]
                .split_ascii_whitespace()
                .filter_map(|component| {
                    if component.is_empty() {
                        return None;
                    }
                    let num = component[..component.len() - 1].parse::<f64>().ok()?;
                    match &component[component.len() - 1..] {
                        "h" => Some(num as i32 * 100 * 60 * 60),
                        "m" => Some(num as i32 * 100 * 60),
                        "s" => Some((num * 100.0).round() as i32),
                        _ => None,
                    }
                })
                .sum::<i32>();

            let old_program_id = components[3];
            let program_id = program_ids[old_program_id];

            let solver_old_id = components[4];
            let solver_id = match solver_ids.entry(solver_old_id) {
                hash_map::Entry::Occupied(e) => *e.get(),
                hash_map::Entry::Vacant(e) => *e.insert(
                    query!(
                        "INSERT INTO UserAccount (name, discord_id) VALUES ($1, $2) RETURNING id",
                        solver_names
                            .get(solver_old_id)
                            .copied()
                            .unwrap_or(solver_old_id),
                        solver_discord_accounts.get(solver_old_id).copied(),
                    )
                    .map(|row| row.id)
                    .fetch_one(&mut *transaction)
                    .await?,
                ),
            };

            let (puzzle_id, variant_id) = puzzle_old_id_to_puzzle_and_variant_id[components[5]];

            let mut average = false;
            let mut blind = false;
            let mut one_handed = false;
            let mut filters = old_program_id == "HSC";
            match components[6] {
                "single" => (),
                "ao5" => average = true,
                "oh" => one_handed = true,
                "bld" => blind = true,
                "nf" => filters = false,
                other => bail!("unknown category {other:?}"),
            }

            query!(
                "INSERT INTO Solve (
                    solver_id, solve_date, upload_date,
                    puzzle_id, variant_id, program_id,
                    average, blind, filters, macros, one_handed, computer_assisted,
                    speed_cs,
                    fmc_verified, speed_verified,
                    video_url
                ) VALUES (
                    $1, $2, $2,
                    $3, $4, $5,
                    $6, $7, $8, FALSE, $9, FALSE,
                    $10,
                    FALSE, TRUE,
                    $11
                )
                ",
                solver_id,
                date,
                puzzle_id,
                variant_id,
                program_id,
                average,
                blind,
                filters,
                one_handed,
                speed_cs,
                video_url
            )
            .execute(&mut *transaction)
            .await?;
        }

        transaction.commit().await?;

        Ok(())
    }
}
