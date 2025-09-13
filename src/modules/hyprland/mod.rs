use saphyr::Yaml;

use crate::utils::Pair;

mod hyprctl;
mod keywords;

/// Main callback to use to apply a YAML node configuration for Hyprland.
///
/// It is expected the function receives directly the root of the Hyprland
/// node, and not the parent one where the Hprland node is stored.
///
/// # Arguments
///
/// * `config` - The Hyprland configuration to explore. See Examples.
///
/// # Return
///
/// A Result object. Ok contains nothing while Error contains the reason of
/// the error as a String.
///
/// # Examples
///
/// This example (once parsed) is expected to be provided to the function :
///
/// ```ignore
/// general:
///   border_size: 1
///   col.inactive_border: rgb(ff0000) rgb(ffff00) 45rad
///   col.active_border: rgb(33ccff) rgb(00ff99) 45rad
/// ```
///
/// But not this (in this case, you should give the value of
/// `config["hyprland"]`) :
///
/// ```ignore
/// hyprland:
///   general:
///     border_size: 1
///     col.inactive_border: rgb(ff0000) rgb(ffff00) 45rad
///     col.active_border: rgb(33ccff) rgb(00ff99) 45rad
/// ```
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
