use anyhow::{anyhow, Context, Result};
use clap::{CommandFactory, FromArgMatches, Parser};
use std::{
    ffi::OsString,
    path::{Path, PathBuf},
    process::Command,
};

#[derive(Parser, Debug)]
#[command(name = "wallpaper")]
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
    let version = "v0.4.1 - fix local and repo mispush";

    let about: &'static str = Box::leak(format!("Set wallpaper and generate palette - {}", version).into_boxed_str());

    let mut cmd = Cli::command();
    cmd = cmd.version(version).about(about);

    let matches = cmd.get_matches();
    let cli = Cli::from_arg_matches(&matches).map_err(|e| anyhow!(e.to_string()))?;

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

fn normalize_and_check_path(original: &Path) -> Result<PathBuf> {
    let expanded = {
        let s = original.to_string_lossy();
        shellexpand::tilde(&s).into_owned()
    };

    let pb = PathBuf::from(expanded);

    if !pb.exists() {
        return Err(anyhow!("Path does not exist: {}", pb.display()));
    }
    if !pb.is_file() {
        return Err(anyhow!("Path is not a file: {}", pb.display()));
    }

    let canonical = pb
        .canonicalize()
        .with_context(|| format!("Failed to canonicalize path: {}", pb.display()))?;

    Ok(canonical)
}

fn run_program(program: &str, args: &[OsString]) -> Result<()> {
    let args_display: Vec<String> = args.iter().map(|a| a.to_string_lossy().into_owned()).collect();
    println!("Running: {} {}", program, args_display.join(" "));

    let mut cmd = Command::new(program);
    cmd.args(args);

    let output = cmd
        .output()
        .with_context(|| format!("Failed to spawn command `{}` (is it installed and in PATH?)", program))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if !stdout.is_empty() {
        print!("{}", stdout);
    }
    if !stderr.is_empty() {
        eprintln!("{}", stderr);
    }

    if !output.status.success() {
        return Err(anyhow!(
            "Command `{}` exited with status: {}",
            program,
            output.status
        ));
    }

    Ok(())
}

fn exec_matugen(path: &Path, matugen_type: &str) -> Result<()> {
    let args: Vec<OsString> = vec![
        OsString::from("image"),
        path.as_os_str().to_os_string(),
        OsString::from("--type"),
        OsString::from(matugen_type),
    ];

    run_program("matugen", &args)
}

fn exec_wal(path: &Path, is_light: bool) -> Result<()> {
    let mut args: Vec<OsString> = vec![OsString::from("run"), path.as_os_str().to_os_string(), OsString::from("-k")];
    if is_light {
        args.push(OsString::from("--palette"));
        args.push(OsString::from("light"));
    }

    run_program("wallust", &args)
}

fn exec_swww(path: &Path) -> Result<()> {
    let args: Vec<OsString> = vec![
        OsString::from("img"),
        path.as_os_str().to_os_string(),
        OsString::from("--transition-type"),
        OsString::from("any"),
        OsString::from("--transition-fps"),
        OsString::from("60"),
        OsString::from("--transition-duration"),
        OsString::from("1"),
    ];

    run_program("swww", &args)
}
