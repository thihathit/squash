use std::path::PathBuf;

use crate::utilities::human_size;

#[derive(Debug, Clone)]
pub struct PathFormatter {
    path: PathBuf,
}

impl PathFormatter {
    pub fn new(path: PathBuf) -> Self {
        assert!(path.is_file(), "PathFormatter requires a file path");
        Self { path }
    }

    pub fn full_path(&self) -> String {
        self.path.display().to_string()
    }

    pub fn file_name(&self) -> String {
        self.path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .into_owned()
    }

    pub fn file_bytes(&self) -> u64 {
        match self.path.metadata() {
            Ok(value) => value.len(),
            Err(_) => 0,
        }
    }

    pub fn size(&self) -> String {
        human_size(self.file_bytes())
    }
}
