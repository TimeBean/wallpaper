use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "wallpaper")]
pub struct Cli {
    /// Use light palette mode
    #[arg(short = 'l', long)]
    pub light: bool,

    /// Open graphical file chooser
    #[arg(long, conflicts_with = "path")]
    pub gui: bool,

    /// matugen scheme type [values: scheme-content, scheme-expressive, scheme-fidelity, scheme-fruit-salad, scheme-monochrome, scheme-neutral, scheme-rainbow, scheme-tonal-spot]
    #[arg(
        long = "type",
        value_name = "TYPE",
        default_value = "scheme-tonal-spot"
    )]
    pub matugen_type: String,

    /// Path to image (ignored if --gui is used)
    pub path: Option<PathBuf>,
}
