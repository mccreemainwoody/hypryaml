use saphyr::{LoadableYamlNode, Yaml};
use std::fs;
use std::path::PathBuf;

use crate::modules;

fn parse_configuration(raw_config: &PathBuf) -> Yaml<'_> {
    let config_string =
        fs::read_to_string(raw_config).expect("failed to read file: ");

    let config = Yaml::load_from_str(&config_string).unwrap();

    Yaml::Sequence(config)
}

pub fn apply_configuration(raw_config: &PathBuf) -> Result<(), String> {
    let config_wrapped = parse_configuration(&raw_config);
    let config = &config_wrapped[0];

    for (key, value) in config.as_mapping().unwrap() {
        let key_string = key.as_str().unwrap();

        match key_string {
            "hyprland" => modules::hyprland::apply_config(&value),
            _ => return Err(format!("{}: {}", "invalid command", key_string)),
        }
    }

    Ok(())
}
