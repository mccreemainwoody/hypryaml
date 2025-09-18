use saphyr::{LoadableYamlNode, ScanError, Yaml};
use std::fs;
use std::path::PathBuf;

use crate::modules;

/// Parse the configuration file. It is assumed the file is written in YAML.
///
/// # Parameters
///
/// * raw_config - The path to a valid YAML configuration.
///
/// # Return
///
/// A Result object. Ok contains he resulting YAML object representation while
/// Error contains a YAML ScanError structure.
///
/// See documentation about the [saphyr](https://github.com/saphyr-rs/saphyr)
/// library for more information about YAML representation structures.
///
/// # Panic
///
/// This function will panic if :
///
/// - The file is not accessible.
/// - The file is not a valid YAML configuration.
fn parse_configuration(raw_config: &PathBuf) -> Result<Yaml<'_>, ScanError> {
    let config_string =
        fs::read_to_string(raw_config).expect("failed to read file: ");

    let config = Yaml::load_from_str(&config_string);

    match config {
        Ok(node) => Ok(Yaml::Sequence(node)),
        Err(error) => Err(error),
    }
}

/// Evaluate and apply the specified configuration.
//
/// Given a configuration at a specified path, parse the configuration,
/// retrieve each valid keyword and provide the corresponding subnode to their
/// application callback.
///
/// The parsing is stopped if an unexpected key is found or if one of the
/// modules returned an erronous result.
///
/// # Parameters
///
/// * raw_config - The path to a valid YAML configuration.
///
/// # Return
///
/// A Result object. Ok contains nothing while Error contains the reason of
/// the error stored as a String.
pub fn apply_configuration(raw_config: &PathBuf) -> Result<(), String> {
    let config_check = parse_configuration(&raw_config);

    if config_check.is_err() {
        let error = config_check.unwrap_err();
        let error_message =
            format!("failed to parse YAML: {}", error.to_string());

        return Err(error_message);
    }

    let config_wrapped = config_check.unwrap();

    let config = &config_wrapped[0];

    if !config.is_mapping() {
        let error_message = format!("supposed YAML config is not a mapping");

        return Err(error_message);
    }

    for (key, value) in config.as_mapping().unwrap() {
        let key_string = key.as_str().unwrap();

        let application_result = match key_string {
            "hyprland" => modules::hyprland::apply_config(&value),
            "hyprpaper" => modules::hyprpaper::apply_config(&value),
            _ => return Err(format!("{}: {}", "invalid command", key_string)),
        };

        if application_result.is_err() {
            return application_result;
        }
    }

    Ok(())
}
