use crate::error::{MorelError, Result};
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

pub enum ReloadResult {
    NoChange,
    Appended,
    Truncated,
    Modified,
    Deleted,
}

pub struct FileReader {
    path: PathBuf,
    content: Vec<String>,
    last_modified: Option<SystemTime>,
}

impl FileReader {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref().to_path_buf();

        if !path.exists() {
            return Err(MorelError::FileNotFound(
                path.display().to_string(),
            ));
        }

        let content = Self::read_file_lines(&path)?;
        let last_modified = fs::metadata(&path)
            .ok()
            .and_then(|m| m.modified().ok());

        Ok(Self {
            path,
            content,
            last_modified,
        })
    }

    fn read_file_lines(path: &Path) -> Result<Vec<String>> {
        let file = fs::File::open(path)?;
        let reader = BufReader::new(file);
        let lines: std::io::Result<Vec<String>> = reader.lines().collect();
        Ok(lines?)
    }

    pub fn reload(&mut self) -> Result<ReloadResult> {
        // Check if file still exists
        if !self.path.exists() {
            return Ok(ReloadResult::Deleted);
        }

        // Get new metadata
        let metadata = fs::metadata(&self.path)?;
        let new_modified = metadata.modified().ok();

        // Compare with cached metadata
        if new_modified == self.last_modified {
            return Ok(ReloadResult::NoChange);
        }

        // Read new content
        let old_len = self.content.len();
        let new_content = Self::read_file_lines(&self.path)?;
        let new_len = new_content.len();

        // Determine change type
        let result = match old_len.cmp(&new_len) {
            std::cmp::Ordering::Less => ReloadResult::Appended,
            std::cmp::Ordering::Greater => ReloadResult::Truncated,
            std::cmp::Ordering::Equal => ReloadResult::Modified,
        };

        // Update state
        self.content = new_content;
        self.last_modified = new_modified;

        Ok(result)
    }

    pub fn get_lines(&self, start: usize, count: usize) -> &[String] {
        let end = (start + count).min(self.content.len());
        let start = start.min(self.content.len());
        &self.content[start..end]
    }

    pub fn total_lines(&self) -> usize {
        self.content.len()
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}
