// Module declarations.
pub mod util;

/// Re-exports `util::command_prelude` to be brought in by
/// `use ffcli::command_prelude`.
pub use util::command_prelude;
pub use util::errors::{FfcliError, Result};
