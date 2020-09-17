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
//!
//! <br>
//!
//! # Tips
//!
//! You'll want to consider what happens when Cargo runs multiple build scripts
//! concurrently that access the same scratch dir. In some use cases you likely
//! want some synchronization over the contents of the scratch directory, such
//! as by an advisory [file lock]. On Unix-like and Windows host systems the
//! simplest way to sequence the build scripts such that each one gets exclusive
//! access one after the other is something like:
//!
//! [file lock]: https://man7.org/linux/man-pages/man2/flock.2.html
//!
//! ```edition2018
//! use std::fs::{self, File};
//! use std::io;
//!
//! fn main() -> io::Result<()> {
//!     let dir = scratch::path("demo");
//!     let _ = fs::create_dir(&dir);
//!     let flock = File::create(dir.join(".lock"))?;
//!     fs2::FileExt::lock_exclusive(&flock)?;
//!
//!     // ... now do work
//!     # Ok(())
//! }
//! ```

use std::path::{Path, PathBuf};

pub fn path(suffix: &str) -> PathBuf {
    Path::new(env!("OUT_DIR")).join(suffix)
}
