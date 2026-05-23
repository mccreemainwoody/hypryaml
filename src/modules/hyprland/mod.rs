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

/// Build a list of keyword pairs based on the provided YAML tree.
///
/// Each leaf of the tree will be referenced as a `Pair` object, with the first
/// value set as the leaf key formatted as `node1:node2:leaf` (assuming the
/// leaf is wrapped inside two nodes, for example), and the second value the
/// value of the leaf formatted as a `String`.
///
/// # Parameters :
///
/// * `tree` - A Yaml tree.
///
/// # Return
///
/// A vector containing of all the name of each leaf along with its value.
fn build_keywords(tree: &Yaml<'_>) -> Vec<Pair<String>> {
    let mut keywords: Vec<Pair<String, String>> = vec![];
    let base_prefix = "".to_owned();

    keywords::generate_keywords(tree, &base_prefix, &mut keywords);

    keywords
}

/// Update all the keywords referenced by the vector by the specified value
/// for the current Hyprland instance.
///
/// Subwrapper for the `apply_config` function.
///
/// # Arguments
///
/// * `keywords` - A vector containing a list of pairs, with the first value
///                being the Hyprland keyword to set and the second value the
///                value to use.
///
/// # Return
///
/// A Result object. Ok contains nothing while Error contains the reason of
/// the error as a String.
fn apply_config_preprocessed(
    keywords: &Vec<Pair<String, String>>,
) -> Result<(), String> {
    println!("Applying hyprland config...");
    hyprctl::apply_keywords_to_config(keywords)
}

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
#[allow(unused)] // NOTE: This interfact is kept for lib usage
pub fn apply_config(config: &Yaml<'_>) -> Result<(), String> {
    let keywords = build_keywords(config);
    apply_config_preprocessed(&keywords)
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
pub fn apply_config_or_restore(config: &Yaml<'_>) -> Result<(), String> {
    let keywords = build_keywords(config);
    let all_keys = keywords.iter().map(|pair| pair.first()).collect();
    let old_config = get_configuration(all_keys)?;

    let result = apply_config_preprocessed(&keywords);

    if let Ok(_) = result {
        return result;
    }

    let reason = result.unwrap_err();
    let restoration = hyprctl::apply_keywords_to_config(&old_config);

    let restore_outcome = match restoration {
        Ok(()) => "Previous configuration has been restored".to_owned(),
        Err(restore_error) => format!(
            "Moreover, failed to restore previous configuration:\n{}",
            restore_error
        ),
    };

    let full_error = format!("{}\n{}", reason, restore_outcome);

    return Err(full_error);
}
