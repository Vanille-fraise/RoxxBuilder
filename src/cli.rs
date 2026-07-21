use clap::Parser;
use std::sync::OnceLock;

/// Command-line arguments. Add new options here as the CLI grows.
#[derive(Parser, Debug, Clone, Default)]
#[command(name = "roxx_builder", version, about = "API that generates optimized Dofus builds")]
pub struct Args {
    /// Enable verbose output
    #[arg(short, long)]
    pub verbose: bool,
}

static ARGS: OnceLock<Args> = OnceLock::new();

/// Parses the command-line arguments and makes them globally available.
/// Must be called once, first thing in `main`.
pub fn init() {
    if ARGS.set(Args::parse()).is_err() {
        panic!("cli::init() called twice");
    }
}

/// Global access to the parsed command-line arguments.
/// Falls back to defaults (everything off) when `init()` was not called (e.g. in tests).
pub fn args() -> &'static Args {
    ARGS.get_or_init(Args::default)
}

/// `true` when `-v` / `--verbose` was passed on the command line.
pub fn is_verbose() -> bool {
    args().verbose
}

/// Like `println!`, but prints only when the `-v` / `--verbose` CLI flag is set.
#[macro_export]
macro_rules! vprintln {
    ($($arg:tt)*) => {
        if $crate::cli::is_verbose() {
            println!($($arg)*);
        }
    };
}
