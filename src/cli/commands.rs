use clap::Subcommand;
use std::path::PathBuf;

use crate::utils::path::validate_path;
use crate::workflow;


#[derive(Subcommand)]
pub enum Commands {
    Apply { config: PathBuf }
}


pub fn run_apply(config: &PathBuf) -> Result<(), String> {
    validate_path(config)
        .and_then(|_| workflow::apply_configuration(&config))
}
