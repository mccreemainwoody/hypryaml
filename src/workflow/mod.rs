use saphyr::{LoadableYamlNode, Yaml};
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
/// The resulting YAML object representation. See documentation about the
/// [saphyr](https://github.com/saphyr-rs/saphyr) library for more information
/// about this structure.
///
/// # Panic
///
/// This function will panic if :
///
/// - The file is not accessible.
/// - The file is not a valid YAML configuration.
fn parse_configuration(raw_config: &PathBuf) -> Yaml<'_> {
    let config_string =
        fs::read_to_string(raw_config).expect("failed to read file: ");

    let config = Yaml::load_from_str(&config_string).unwrap();

    Yaml::Sequence(config)
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
    let config_wrapped = parse_configuration(&raw_config);
    let config = &config_wrapped[0];

    for (key, value) in config.as_mapping().unwrap() {
        let key_string = key.as_str().unwrap();

        let application_result = match key_string {
            "hyprland" => modules::hyprland::apply_config(&value),
            _ => return Err(format!("{}: {}", "invalid command", key_string)),
        };

        if application_result.is_err() {
            return application_result;
        }
    }

    Ok(())
}
