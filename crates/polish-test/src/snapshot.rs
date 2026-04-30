use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;

#[derive(Debug, Clone)]
pub struct Snapshot {
    pub name: String,
    pub content: String,
}

pub struct SnapshotStore {
    dir: PathBuf,
    update: bool,
}

impl SnapshotStore {
    pub fn new(dir: impl Into<PathBuf>, update: bool) -> Self {
        Self { dir: dir.into(), update }
    }

    fn path(&self, name: &str) -> PathBuf {
        self.dir.join(format!("{name}.snap"))
    }

    pub fn assert_matches(&self, name: &str, actual: &str) -> Result<(), String> {
        let path = self.path(name);
        if self.update || !path.exists() {
            fs::create_dir_all(&self.dir).map_err(|e| e.to_string())?;
            fs::write(&path, actual).map_err(|e| e.to_string())?;
            return Ok(());
        }
        let expected = fs::read_to_string(&path).map_err(|e| e.to_string())?;
        if actual == expected {
            Ok(())
        } else {
            Err(format!(
                "Snapshot mismatch for '{name}':\n--- expected ---\n{expected}\n--- actual ---\n{actual}"
            ))
        }
    }

    pub fn list(&self) -> Vec<String> {
        let Ok(rd) = fs::read_dir(&self.dir) else { return Vec::new() };
        rd.filter_map(|e| {
            let e = e.ok()?;
            let name = e.file_name().to_string_lossy().to_string();
            name.strip_suffix(".snap").map(|s| s.to_string())
        })
        .collect()
    }
}
