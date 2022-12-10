use std::path::{PathBuf, Path};

pub enum CreationError {
    PathDoesNotExist,
}

/// Assures that the path exists at the moment of structure creation
pub struct ExistingPath(PathBuf);

impl ExistingPath {
    pub fn new(path: PathBuf) -> Result<Self, (CreationError, PathBuf)> {
        if path.exists() {
            Ok(Self(path))
        } else {
            Err((CreationError::PathDoesNotExist, path))
        }
    }

    pub fn contents(&self) -> &Path {
        &self.0
    }
}
