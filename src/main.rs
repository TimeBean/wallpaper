use anyhow::{Result, anyhow};
use clap::{CommandFactory, FromArgMatches};

mod cli;
mod history;
mod programs;
mod utils;

use cli::Cli;
use history::{add_to_history, display_history, restore_from_history};
use programs::{exec_matugen_with_dry_run, exec_swww_with_dry_run, exec_wal_with_dry_run};

use utils::normalize_and_check_path;

fn main() -> Result<()> {
    let version = "v0.5 - code base rewrite + validation + arg parsing fixed";

    let about: &'static str =
        Box::leak(format!("Set wallpaper and generate palette - {}", version).into_boxed_str());

    let mut cmd = Cli::command();
    cmd = cmd.version(version).about(about);

    let matches = cmd.get_matches();
    let cli = Cli::from_arg_matches(&matches).map_err(|e| anyhow!(e.to_string()))?;

    // Validation: exactly one main option must be provided (dry-run is a modifier)
    let main_options_count = cli.gui as u8
        + cli.path.is_some() as u8
        + cli.history as u8
        + (cli.restore_step != 0) as u8;

    if main_options_count != 1 {
        return Err(anyhow!(
            "Exactly one of --gui, path, --history, or --restore must be provided."
        ));
    }

    if cli.history {
        return display_history();
    }

    if cli.restore_step != 0 {
        let entry = restore_from_history(cli.restore_step)?;
        println!(
            "Restoring wallpaper from step {}: {}",
            cli.restore_step,
            entry.path.display()
        );

        let path = normalize_and_check_path(&entry.path)?;

        exec_swww_with_dry_run(&path, cli.dry_run)?;
        exec_matugen_with_dry_run(&path, &entry.matugen_type, cli.dry_run)?;
        exec_wal_with_dry_run(&path, entry.is_light, cli.dry_run)?;

        // Add restored wallpaper to history as most recent
        if !cli.dry_run {
            add_to_history(&path, &entry.matugen_type, entry.is_light)?;
        }

        println!("Wallpaper restored successfully.");
        return Ok(());
    }

    let raw_path = if cli.gui {
        rfd::FileDialog::new()
            .set_title("Choose wallpaper")
            .pick_file()
            .ok_or_else(|| anyhow!("No file selected via GUI"))?
    } else {
        cli.path
            .ok_or_else(|| anyhow!("No path provided. Supply a path."))?
    };

    let path = normalize_and_check_path(&raw_path)?;

    println!("wallpaper {} - {}", version, path.display());

    exec_swww_with_dry_run(&path, cli.dry_run)?;
    exec_matugen_with_dry_run(&path, &cli.matugen_type, cli.dry_run)?;
    exec_wal_with_dry_run(&path, cli.light, cli.dry_run)?;

    // Add to history (only if not dry run)
    if !cli.dry_run {
        add_to_history(&path, &cli.matugen_type, cli.light)?;
    }

    println!("Done.");
    Ok(())
}
