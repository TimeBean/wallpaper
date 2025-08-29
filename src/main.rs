use std::env;
use std::process::Command;

const VERSION: &str = "v0.2 - file dialog update";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.iter().any(|a| a == "--help" || a == "-h") {
        println!("wallpaper {}\n", VERSION);
        println!("Usage:");
        println!(" wallpaper [OPTIONS] <path>");
        println!(" wallpaper [OPTIONS] --gui");
        println!("");
        println!("Options:");
        println!(" -h, --help Show this help message");
        println!(" -v, --version Show version");
        println!(" --gui Open graphical file chooser to pick an image (uses rfd)");
        println!(" -l Use light palette mode for wallust/wal");
        println!(" --type <scheme-...> Specify matugen color scheme, e.g. --type scheme-tonal-spot");
        println!("Examples:");
        println!(" wallpaper /path/to/image.jpg");
        println!(" wallpaper -l --type scheme-tonal-spot /path/to/image.jpg");
        println!(" wallpaper --gui");
        std::process::exit(0);
    }

    if args.iter().any(|a| a == "--version" || a == "-v") {
        println!("{}", VERSION);
        std::process::exit(0);
    }

    let has_gui = args.iter().any(|a| a == "--gui");
    let has_l = args.iter().any(|a| a == "-l");

    let mut matugen_type = "scheme-tonal-spot".to_string();
    if let Some(scheme_arg) = args.iter().find(|w| w.contains("scheme")) {
        println!("Matugen will use {} color scheme.", scheme_arg);
        matugen_type = scheme_arg.clone();
    }

    let path: String = if has_gui {
        if let Some(pathbuf) = rfd::FileDialog::new()
            .set_title("Choose wallpaper")
            .pick_file()
        {
            pathbuf.to_string_lossy().to_string()
        } else {
            eprintln!("No file selected. Choose a wallpaper (без негатива).");
            std::process::exit(1);
        }
    } else {
        match args.iter().skip(1).find(|a| !a.starts_with('-')) {
            Some(p) => p.clone(),
            None => {
                eprintln!("Usage: {} <path> [-l] [--type scheme-<type>] [--gui]", args[0]);
                std::process::exit(1);
            }
        }
    };

    println!("wallpaper v0.1 - {}", path);

    exec_swww(&path);
    exec_wal(&path, has_l);
    exec_matugen(&path, &matugen_type);

    println!("Done.");
}

fn exec_matugen(path: &str, matugen_type: &str) {
    let mut matugen_cmd = Command::new("matugen");

    matugen_cmd.arg("image").arg(path).arg("--type").arg(matugen_type);

    let matugen = matugen_cmd
        .output()
        .expect("matugen command failed to start");

    println!(
        "\n{}\n{}",
        String::from_utf8_lossy(&matugen.stdout),
        String::from_utf8_lossy(&matugen.stderr)
    );
}

fn exec_wal(path: &str, is_light: bool) {
    let mut wal_cmd = Command::new("wallust");
    wal_cmd.arg("run").arg(path).arg("-k");
    if is_light {
        wal_cmd.arg("--palette").arg("light");
    }

    let wal = wal_cmd
        .output()
        .expect("wal command failed to start");

    println!(
        "\n{}\n{}",
        String::from_utf8_lossy(&wal.stdout),
        String::from_utf8_lossy(&wal.stderr)
    );
}

fn exec_swww(path: &str) {
    let swww = Command::new("swww")
        .arg("img")
        .arg(path)
        .arg("--transition-type")
        .arg("any")
        .arg("--transition-fps")
        .arg("60")
        .arg("--transition-duration")
        .arg("1")
        .output()
        .expect("swww command failed to start");

    println!(
        "\n{}\n{}",
        String::from_utf8_lossy(&swww.stdout),
        String::from_utf8_lossy(&swww.stderr)
    );
}
