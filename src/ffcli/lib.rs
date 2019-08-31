use std::path::{Path, PathBuf};

// Module declarations.
mod util;

/// Re-exports `util::command_prelude` to be brought in by
/// `use kvs::command_prelude`.
pub use util::command_prelude;
pub use util::errors::{FfcliError, Result};

pub struct Project {
    root: PathBuf,
    bin: PathBuf,
    lib: PathBuf,
}

impl Project {
}