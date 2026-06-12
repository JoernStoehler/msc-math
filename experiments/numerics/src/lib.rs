pub mod args;
pub mod audit;
pub mod events;
pub mod output;

use std::path::PathBuf;

pub fn run_from_env() -> Result<PathBuf, String> {
    let config = args::parse_args(std::env::args().skip(1))?;
    audit::run(config)
}
