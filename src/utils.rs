use anyhow::{Context, Result, anyhow};
use std::{
    ffi::OsString,
    path::{Path, PathBuf},
    process::Command,
};

pub fn normalize_and_check_path(original: &Path) -> Result<PathBuf> {
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

pub fn run_program(program: &str, args: &[OsString]) -> Result<()> {
    let args_display: Vec<String> = args
        .iter()
        .map(|a| a.to_string_lossy().into_owned())
        .collect();
    println!("Running: {} {}", program, args_display.join(" "));

    let mut cmd = Command::new(program);
    cmd.args(args);

    let output = cmd.output().with_context(|| {
        format!(
            "Failed to spawn command `{}` (is it installed and in PATH?)",
            program
        )
    })?;

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
