use anyhow::{Result, anyhow};
use clap::{CommandFactory, FromArgMatches};

mod cli;
mod programs;
mod utils;

use cli::Cli;
use programs::{exec_matugen, exec_swww, exec_wal};
use utils::normalize_and_check_path;

fn main() -> Result<()> {
    let version = "v0.5 - code base rewrite + validation + arg parsing fixed";

    let about: &'static str =
        Box::leak(format!("Set wallpaper and generate palette - {}", version).into_boxed_str());

    let mut cmd = Cli::command();
    cmd = cmd.version(version).about(about);

    let matches = cmd.get_matches();
    let cli = Cli::from_arg_matches(&matches).map_err(|e| anyhow!(e.to_string()))?;

    // Validation: either --gui or path must be provided
    if !cli.gui && cli.path.is_none() {
        return Err(anyhow!("Either --gui or a path must be provided"));
    }

    let raw_path = if cli.gui {
        rfd::FileDialog::new()
            .set_title("Choose wallpaper")
            .pick_file()
            .ok_or_else(|| anyhow!("No file selected via GUI"))?
    } else {
        cli.path
            .ok_or_else(|| anyhow!("No path provided. Use --gui or supply a path."))?
    };

    let path = normalize_and_check_path(&raw_path)?;

    println!("wallpaper {} - {}", version, path.display());

    exec_swww(&path)?;
    exec_matugen(&path, &cli.matugen_type)?;
    exec_wal(&path, cli.light)?;

    println!("Done.");
    Ok(())
}
