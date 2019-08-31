/// Re-exporting `std::process`.
pub use std::process;

/// Re-exporting commonly necessary types from [`clap`].(https://github.com/clap-rs/clap)
pub use clap::{AppSettings, Arg, ArgMatches, SubCommand};

/// Type alias for a `clap::App`.
pub type App = clap::App<'static, 'static>;
