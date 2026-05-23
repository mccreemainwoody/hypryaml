use saphyr::Yaml;

use crate::utils::Pair;

mod hyprctl;
mod keywords;

/// Retrieve the values set for each requested keyword in the current Hyprland
/// instance.
///
/// The values are returned as a `Pair` vector for each keyword along with its
/// value.
///
/// # Parameters
///
/// * `keyword` - A vector containing each keyword to query as a string.
///
/// # Return
///
/// A Result object. Ok contains a vector of `Pair`s as described in the
/// description. Err along with the reason is returned if something wrong
/// happened with the querying of the value from Hyprland.
fn get_configuration(
    keywords: Vec<&String>,
) -> Result<Vec<Pair<String>>, String> {
    let mut config: Vec<Pair<String>> = vec![];

    for keyword in keywords {
        let key: String = keyword.to_string();
        let value = hyprctl::get_option(&key);

        match value {
            Ok(value) => {
                let pair = Pair::new(key, value);
                config.push(pair);
            }
            Err(reason) => return Err(reason),
        }
    }

    Ok(config)
}

/// Main callback to use to apply a YAML node configuration for Hyprland.
///
/// It is expected the function receives directly the root of the Hyprland
/// node, and not the parent one where the Hprland node is stored.
///
/// If the configuration update has failed, all the keywords that were
/// supposed to be set will be restored to their original value.
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
    println!("Applying hyprland config...");

    let mut keywords: Vec<Pair<String, String>> = vec![];
    let base_prefix = String::from("");

    keywords::generate_keywords(config, &base_prefix, &mut keywords);

    let all_keys = keywords.iter().map(|pair| pair.first()).collect();
    let old_config = get_configuration(all_keys)?;
    let result = hyprctl::apply_keywords_to_config(keywords);

    match result {
        Ok(_status_code) => Ok(()),
        Err(error) => {
            let reason = error.to_string();
            let restoration = hyprctl::apply_keywords_to_config(old_config);

            let restore_outcome = match restoration {
                Ok(()) => "Previous configuration has been restored".to_owned(),
                Err(restore_error) => format!(
                    "Moreover, failed to restore previous configuration:\n{}",
                    restore_error.to_string()
                ),
            };

            let full_error = format!("{}\n{}", reason, restore_outcome);

            Err(full_error)
        }
    }
}
