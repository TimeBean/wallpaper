use anyhow::Result;
use std::{ffi::OsString, path::Path};

use crate::utils::run_program;

#[derive(Debug)]
pub enum Program {
    Matugen {
        path: Box<Path>,
        matugen_type: String,
    },
    Wallust {
        path: Box<Path>,
        is_light: bool,
    },
    Swww {
        path: Box<Path>,
    },
}

impl Program {
    pub fn matugen(path: &Path, matugen_type: &str) -> Self {
        Self::Matugen {
            path: path.into(),
            matugen_type: matugen_type.to_string(),
        }
    }

    pub fn wallust(path: &Path, is_light: bool) -> Self {
        Self::Wallust {
            path: path.into(),
            is_light,
        }
    }

    pub fn swww(path: &Path) -> Self {
        Self::Swww { path: path.into() }
    }

    fn get_program_name(&self) -> &'static str {
        match self {
            Program::Matugen { .. } => "matugen",
            Program::Wallust { .. } => "wallust",
            Program::Swww { .. } => "swww",
        }
    }

    fn get_args(&self) -> Vec<OsString> {
        match self {
            Program::Matugen { path, matugen_type } => {
                vec![
                    OsString::from("image"),
                    path.as_os_str().to_os_string(),
                    OsString::from("--type"),
                    OsString::from(matugen_type),
                ]
            }
            Program::Wallust { path, is_light } => {
                let mut args = vec![
                    OsString::from("run"),
                    path.as_os_str().to_os_string(),
                    OsString::from("-k"),
                ];
                if *is_light {
                    args.push(OsString::from("--palette"));
                    args.push(OsString::from("light"));
                }
                args
            }
            Program::Swww { path } => {
                vec![
                    OsString::from("img"),
                    path.as_os_str().to_os_string(),
                    OsString::from("--transition-type"),
                    OsString::from("any"),
                    OsString::from("--transition-fps"),
                    OsString::from("60"),
                    OsString::from("--transition-duration"),
                    OsString::from("1"),
                ]
            }
        }
    }

    pub fn execute(&self) -> Result<()> {
        let program_name = self.get_program_name();
        let args = self.get_args();
        run_program(program_name, &args)
    }
}

pub fn exec_matugen(path: &Path, matugen_type: &str) -> Result<()> {
    Program::matugen(path, matugen_type).execute()
}

pub fn exec_wal(path: &Path, is_light: bool) -> Result<()> {
    Program::wallust(path, is_light).execute()
}

pub fn exec_swww(path: &Path) -> Result<()> {
    Program::swww(path).execute()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_program_matugen_creation() {
        let path = Path::new("/test/image.jpg");
        let program = Program::matugen(path, "scheme-tonal-spot");

        match program {
            Program::Matugen {
                path: p,
                matugen_type,
            } => {
                assert_eq!(p.as_ref(), Path::new("/test/image.jpg"));
                assert_eq!(matugen_type, "scheme-tonal-spot");
            }
            _ => panic!("Expected Matugen variant"),
        }
    }

    #[test]
    fn test_program_wallust_creation() {
        let path = Path::new("/test/image.jpg");
        let program = Program::wallust(path, true);

        match program {
            Program::Wallust { path: p, is_light } => {
                assert_eq!(p.as_ref(), Path::new("/test/image.jpg"));
                assert!(is_light);
            }
            _ => panic!("Expected Wallust variant"),
        }
    }

    #[test]
    fn test_program_swww_creation() {
        let path = Path::new("/test/image.jpg");
        let program = Program::swww(path);

        match program {
            Program::Swww { path: p } => {
                assert_eq!(p.as_ref(), Path::new("/test/image.jpg"));
            }
            _ => panic!("Expected Swww variant"),
        }
    }

    #[test]
    fn test_program_names() {
        let path = Path::new("/test/image.jpg");

        assert_eq!(Program::matugen(path, "test").get_program_name(), "matugen");
        assert_eq!(Program::wallust(path, false).get_program_name(), "wallust");
        assert_eq!(Program::swww(path).get_program_name(), "swww");
    }

    #[test]
    fn test_matugen_args() {
        let path = Path::new("/test/image.jpg");
        let program = Program::matugen(path, "scheme-content");
        let args = program.get_args();

        assert_eq!(args.len(), 4);
        assert_eq!(args[0], "image");
        assert_eq!(args[1], "/test/image.jpg");
        assert_eq!(args[2], "--type");
        assert_eq!(args[3], "scheme-content");
    }

    #[test]
    fn test_wallust_args_dark() {
        let path = Path::new("/test/image.jpg");
        let program = Program::wallust(path, false);
        let args = program.get_args();

        assert_eq!(args.len(), 3);
        assert_eq!(args[0], "run");
        assert_eq!(args[1], "/test/image.jpg");
        assert_eq!(args[2], "-k");
    }

    #[test]
    fn test_wallust_args_light() {
        let path = Path::new("/test/image.jpg");
        let program = Program::wallust(path, true);
        let args = program.get_args();

        assert_eq!(args.len(), 5);
        assert_eq!(args[0], "run");
        assert_eq!(args[1], "/test/image.jpg");
        assert_eq!(args[2], "-k");
        assert_eq!(args[3], "--palette");
        assert_eq!(args[4], "light");
    }

    #[test]
    fn test_swww_args() {
        let path = Path::new("/test/image.jpg");
        let program = Program::swww(path);
        let args = program.get_args();

        assert_eq!(args.len(), 8);
        assert_eq!(args[0], "img");
        assert_eq!(args[1], "/test/image.jpg");
        assert_eq!(args[2], "--transition-type");
        assert_eq!(args[3], "any");
        assert_eq!(args[4], "--transition-fps");
        assert_eq!(args[5], "60");
        assert_eq!(args[6], "--transition-duration");
        assert_eq!(args[7], "1");
    }
}
