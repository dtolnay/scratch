//! This crate exposes a compile-time temporary directory sharable by multiple
//! crates in a build graph and erased by `cargo clean`.
//!
//! The intended usage is from a build.rs Cargo build script:
//!
//! ```toml
//! # Cargo.toml
//!
//! [build-dependencies]
//! scratch = "0.0"
//! ```
//!
//! ```
//! // build.rs
//!
//! fn main() {
//!     let dir = scratch::path("mycrate");
//!     // ... write or read inside of that path
//! }
//! ```

use std::path::{Path, PathBuf};

pub fn path(suffix: &str) -> PathBuf {
    Path::new(env!("OUT_DIR")).join(suffix)
}
