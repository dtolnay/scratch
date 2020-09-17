use std::path::{Path, PathBuf};

pub fn path(suffix: &str) -> PathBuf {
    Path::new(env!("OUT_DIR")).join(suffix)
}
