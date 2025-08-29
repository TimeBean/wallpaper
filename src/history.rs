use anyhow::{Context, Result, anyhow};
use serde::{Deserialize, Serialize};
use std::{
    collections::VecDeque,
    fs,
    path::{Path, PathBuf},
};

const MAX_HISTORY_ENTRIES: usize = 50;
const HISTORY_FILENAME: &str = "history.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WallpaperEntry {
    pub path: PathBuf,
    pub timestamp: u64,
    pub matugen_type: String,
    pub is_light: bool,
}

impl WallpaperEntry {
    pub fn new(path: PathBuf, matugen_type: String, is_light: bool) -> Self {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            path,
            timestamp,
            matugen_type,
            is_light,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WallpaperHistory {
    entries: VecDeque<WallpaperEntry>,
}

impl Default for WallpaperHistory {
    fn default() -> Self {
        Self {
            entries: VecDeque::with_capacity(MAX_HISTORY_ENTRIES),
        }
    }
}

impl WallpaperHistory {
    pub fn load() -> Result<Self> {
        let history_path = get_history_file_path()?;

        if !history_path.exists() {
            return Ok(Self::default());
        }

        let content = fs::read_to_string(&history_path)
            .with_context(|| format!("Failed to read history file: {}", history_path.display()))?;

        let history: WallpaperHistory =
            serde_json::from_str(&content).with_context(|| "Failed to parse history file")?;

        Ok(history)
    }

    pub fn save(&self) -> Result<()> {
        let history_path = get_history_file_path()?;

        // Ensure parent directory exists
        if let Some(parent) = history_path.parent() {
            fs::create_dir_all(parent).with_context(|| {
                format!("Failed to create history directory: {}", parent.display())
            })?;
        }

        let content =
            serde_json::to_string_pretty(self).with_context(|| "Failed to serialize history")?;

        fs::write(&history_path, content)
            .with_context(|| format!("Failed to write history file: {}", history_path.display()))?;

        Ok(())
    }

    pub fn add_entry(&mut self, path: PathBuf, matugen_type: String, is_light: bool) {
        let entry = WallpaperEntry::new(path, matugen_type, is_light);

        // Remove duplicate if exists
        self.entries.retain(|e| e.path != entry.path);

        // Add to front
        self.entries.push_front(entry);

        // Keep only MAX_HISTORY_ENTRIES
        if self.entries.len() > MAX_HISTORY_ENTRIES {
            self.entries.pop_back();
        }
    }

    pub fn get_entries(&self) -> &VecDeque<WallpaperEntry> {
        &self.entries
    }

    pub fn get_entry(&self, index: usize) -> Option<&WallpaperEntry> {
        self.entries.get(index)
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }
}

fn get_history_file_path() -> Result<PathBuf> {
    let data_dir = get_data_directory()?;
    Ok(data_dir.join(HISTORY_FILENAME))
}

fn get_data_directory() -> Result<PathBuf> {
    // Try to use XDG_DATA_HOME first, fallback to ~/.local/share
    if let Ok(xdg_data_home) = std::env::var("XDG_DATA_HOME") {
        Ok(PathBuf::from(xdg_data_home).join("wallpaper"))
    } else if let Ok(home) = std::env::var("HOME") {
        Ok(PathBuf::from(home).join(".local/share/wallpaper"))
    } else {
        Err(anyhow!(
            "Unable to determine data directory. HOME environment variable not set."
        ))
    }
}

pub fn add_to_history(path: &Path, matugen_type: &str, is_light: bool) -> Result<()> {
    let mut history = WallpaperHistory::load()?;
    history.add_entry(path.to_path_buf(), matugen_type.to_string(), is_light);
    history.save()?;
    Ok(())
}

pub fn display_history() -> Result<()> {
    let history = WallpaperHistory::load()?;

    if history.is_empty() {
        println!("No wallpaper history found.");
        return Ok(());
    }

    println!("Wallpaper History ({} entries):", history.len());
    println!("{:-<80}", "");

    for (index, entry) in history.get_entries().iter().enumerate() {
        let timestamp = std::time::UNIX_EPOCH + std::time::Duration::from_secs(entry.timestamp);
        let datetime = humantime::format_rfc3339_seconds(timestamp);

        println!(
            "{:2}: {} | {} | Type: {} | Light: {}",
            index + 1,
            entry.path.display(),
            datetime,
            entry.matugen_type,
            entry.is_light
        );
    }

    Ok(())
}

pub fn restore_from_history(step: i32) -> Result<WallpaperEntry> {
    let history = WallpaperHistory::load()?;

    if history.is_empty() {
        return Err(anyhow!("No wallpaper history found."));
    }

    let index = if step > 0 {
        (step - 1) as usize // 1-based to 0-based
    } else {
        return Err(anyhow!("Restore step must be positive (1-based indexing)."));
    };

    let entry = history.get_entry(index).ok_or_else(|| {
        anyhow!(
            "Invalid restore step: {}. History has {} entries.",
            step,
            history.len()
        )
    })?;

    // Check if the file still exists
    if !entry.path.exists() {
        return Err(anyhow!(
            "Wallpaper file no longer exists: {}",
            entry.path.display()
        ));
    }

    Ok(entry.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wallpaper_entry_creation() {
        let path = PathBuf::from("/test/image.jpg");
        let entry = WallpaperEntry::new(path.clone(), "scheme-tonal-spot".to_string(), false);

        assert_eq!(entry.path, path);
        assert_eq!(entry.matugen_type, "scheme-tonal-spot");
        assert!(!entry.is_light);
        assert!(entry.timestamp > 0);
    }

    #[test]
    fn test_history_add_entry() {
        let mut history = WallpaperHistory::default();
        let path = PathBuf::from("/test/image.jpg");

        history.add_entry(path.clone(), "scheme-tonal-spot".to_string(), false);

        assert_eq!(history.len(), 1);
        assert_eq!(history.get_entry(0).unwrap().path, path);
    }

    #[test]
    fn test_history_duplicate_removal() {
        let mut history = WallpaperHistory::default();
        let path = PathBuf::from("/test/image.jpg");

        history.add_entry(path.clone(), "scheme-tonal-spot".to_string(), false);
        history.add_entry(path.clone(), "scheme-content".to_string(), true);

        assert_eq!(history.len(), 1);
        assert_eq!(history.get_entry(0).unwrap().matugen_type, "scheme-content");
        assert!(history.get_entry(0).unwrap().is_light);
    }

    #[test]
    fn test_history_max_entries() {
        let mut history = WallpaperHistory::default();

        // Add more than MAX_HISTORY_ENTRIES
        for i in 0..MAX_HISTORY_ENTRIES + 10 {
            let path = PathBuf::from(format!("/test/image{}.jpg", i));
            history.add_entry(path, "scheme-tonal-spot".to_string(), false);
        }

        assert_eq!(history.len(), MAX_HISTORY_ENTRIES);
    }

    #[test]
    fn test_serialize_deserialize() {
        let mut history = WallpaperHistory::default();
        let path = PathBuf::from("/test/image.jpg");
        history.add_entry(path.clone(), "scheme-tonal-spot".to_string(), false);

        let serialized = serde_json::to_string(&history).unwrap();
        let deserialized: WallpaperHistory = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.len(), 1);
        assert_eq!(deserialized.get_entry(0).unwrap().path, path);
    }
}
