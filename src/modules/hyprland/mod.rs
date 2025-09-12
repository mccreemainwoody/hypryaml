use saphyr::Yaml;

use crate::utils::Pair;

mod hyprctl;
mod keywords;

pub fn apply_config(config: &Yaml<'_>) -> Result<(), String> {
    println!("applying hyprland config...");

    let mut keywords: Vec<Pair<String, String>> = vec![];
    let base_prefix = String::from("");

    keywords::generate_keywords(config, &base_prefix, &mut keywords);

    let result = hyprctl::apply_keywords_to_config(keywords);

    match result {
        Ok(_status_code) => Ok(()),
        Err(error) => Err(error.to_string()),
    }
}
