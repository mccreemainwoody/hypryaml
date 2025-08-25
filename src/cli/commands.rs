use clap::Subcommand;
use std::path::PathBuf;

use crate::workflow;


#[derive(Subcommand)]
pub enum Commands {
    Apply { config: PathBuf }
}


pub fn run_apply(config: &PathBuf) -> Result<(), &str> {
    workflow::apply_configuration(&config);

    Ok(())
}
