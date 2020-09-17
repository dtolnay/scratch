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
//!
//! <br>
//!
//! # Comparisons
//!
//! Comparison to **`std::env::var_os("OUT_DIR")`**:
//!
//! - This functionality is different from OUT_DIR in that the same directory
//!   path will be seen by *all* crates whose build passes a matching `suffix`
//!   argument, and each crate can see content placed into the directory by
//!   those other crates' build scripts that have already run.
//!
//! - This functionality is similar to OUT_DIR in that both are erased when a
//!   `cargo clean` is executed.
//!
//! Comparison to **`std::env::temp_dir()`**:
//!
//! - This functionality is similar to temp_dir() in that stuff that goes in is
//!   visible to subsequently running build scripts.
//!
//! - This functionality is different from temp_dir() in that `cargo clean`
//!   cleans up the contents.

use std::path::{Path, PathBuf};

pub fn path(suffix: &str) -> PathBuf {
    Path::new(env!("OUT_DIR")).join(suffix)
}
