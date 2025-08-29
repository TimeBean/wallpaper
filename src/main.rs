use anyhow::{anyhow, Context, Result};
use clap::Parser;
use std::{path::PathBuf, process::Command};

const VERSION: &str = "v0.3 - code base + order update";

#[derive(Parser, Debug)]
#[command(name = "wallpaper", version = VERSION, about = "Set wallpaper and generate palette")]
struct Cli {
    /// Use light palette mode
    #[arg(short = 'l', long)]
    light: bool,

    /// Open graphical file chooser
    #[arg(long)]
    gui: bool,

    /// matugen scheme type [values: scheme-content, scheme-expressive, scheme-fidelity, scheme-fruit-salad, scheme-monochrome, scheme-neutral, scheme-rainbow, scheme-tonal-spot]
    #[arg(long = "type", value_name = "TYPE", default_value = "scheme-tonal-spot")]
    matugen_type: String,

    /// Path to image (ignored if --gui is used)
    path: Option<PathBuf>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let path_buf = if cli.gui {
        rfd::FileDialog::new()
            .set_title("Choose wallpaper")
            .pick_file()
            .ok_or_else(|| anyhow!("No file selected via GUI"))?
    } else {
        cli.path
            .ok_or_else(|| anyhow!("No path provided. Use --gui or supply a path."))?
    };

    println!("wallpaper {} - {}", VERSION, path_buf.display());

    exec_swww(&path_buf)?;
    exec_matugen(&path_buf, &cli.matugen_type)?;
    exec_wal(&path_buf, cli.light)?;

    println!("Done.");
    Ok(())
}

fn run_cmd_and_log(cmd: &mut Command) -> Result<()> {
    let output = cmd
        .output()
        .with_context(|| format!("failed to spawn command {:?}", cmd))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if !stdout.is_empty() {
        println!("{}", stdout);
    }
    if !stderr.is_empty() {
        eprintln!("{}", stderr);
    }

    if !output.status.success() {
        return Err(anyhow!(
            "Command {:?} exited with code: {}",
            cmd,
            output
                .status
                .code()
                .map(|c| c.to_string())
                .unwrap_or_else(|| "unknown".into())
        ));
    }
    Ok(())
}

fn exec_matugen(path: &PathBuf, matugen_type: &str) -> Result<()> {
    let mut cmd = Command::new("matugen");
    cmd.arg("image")
        .arg(path)
        .arg("--type")
        .arg(matugen_type);

    run_cmd_and_log(&mut cmd)
}

fn exec_wal(path: &PathBuf, is_light: bool) -> Result<()> {
    let mut cmd = Command::new("wallust");
    cmd.arg("run").arg(path).arg("-k");
    if is_light {
        cmd.arg("--palette").arg("light");
    }

    run_cmd_and_log(&mut cmd)
}

fn exec_swww(path: &PathBuf) -> Result<()> {
    let mut cmd = Command::new("swww");
    cmd.arg("img")
        .arg(path)
        .arg("--transition-type")
        .arg("any")
        .arg("--transition-fps")
        .arg("60")
        .arg("--transition-duration")
        .arg("1");

    run_cmd_and_log(&mut cmd)
}
