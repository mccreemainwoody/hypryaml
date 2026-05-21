use clap::Subcommand;
use std::path::PathBuf;

use crate::modules::notify;
use crate::utils::validate_path;
use crate::workflow;

/// Define the API structure of the command line interface.
#[derive(Subcommand)]
pub enum Commands {
    Apply { config: PathBuf },
}

/// Main callback of the Apply command.
///
/// This validate and parse the provided configuration file as YAML and apply
/// it to all the specified tools.
///
/// # Arguments
///
/// * `config` - A path to the configuration to apply. The path **must** be
///              valid.
///
/// # Return
///
/// A Result object. Ok contains nothing, while Error will contain the reason
/// of the error as a String.
pub fn run_apply(config: &PathBuf) -> Result<(), String> {
    let result = validate_path(config)
        .and_then(|_| workflow::apply_configuration(&config));

    let notify_content = match result {
        Ok(_) => Ok("configuration succesfully applied"),
        Err(ref reason) => Err(reason.as_str()),
    };
    let notify_timeout: u32 = 5000;

    notify::send_notification(&notify_content, &notify_timeout);

    result
}
