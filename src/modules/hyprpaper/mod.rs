use saphyr::{Sequence, Yaml};

use crate::{
    modules::hyprpaper::hyprctl::get_wallpapers,
    utils::{Pair, error::create_error},
};

mod hyprctl;

/// Return whether all the YAML nodes of the Sequence are mappings.
///
/// # Arguments
///
/// * `sequence` - The Yaml sequence to validate.
///
/// # Return
///
/// true if all the elements of the sequence are of enum type Yaml::Mapping.
/// false otherwise.
fn validate_config_sequence(sequence: &Sequence) -> bool {
    sequence.iter().all(|subconfig| subconfig.is_mapping())
}

/// Main callback to use to apply a YAML node configuration for hyprpaper.
///
/// It is expected the function receives directly the root of the hyprpaper
/// node, and not the parent one where the hyprpaper node is stored.
///
/// When this function is called, the current wallpapers loaded for each
/// montor to update will be retrieved. If the update fails for at least one
/// wallpaper, the whole application is cancelled and the previous
/// configuration is loaded back into the hyprpaper runtime.
///
/// # Arguments
///
/// * `config` - The hyprpaper configuration to explore. See Examples.
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
/// - monitor: DPD-1
///   background: /home/me/amogus.jpg
/// - monitor: DPD-2
///   background: /home/me/sugoma.jpg
/// ```
///
/// But not this (in this case, you should give the value of
/// `config["hyprpaper"]`) :
///
/// ```ignore
/// hyprpaper:
///   - monitor: DPD-1
///     background: /home/me/amogus.jpg
///   - monitor: DPD-2
///     background: /home/me/sugoma.jpg
/// ```
pub fn apply_config(config: &Yaml<'_>) -> Result<(), String> {
    if !config.is_sequence() {
        return create_error("hyprpaper configuration should be a sequence");
    }

    let wallpapers = config.as_sequence().unwrap();

    if !validate_config_sequence(wallpapers) {
        return create_error(
            "hyprpaper configuration must be a sequence of mappings",
        );
    }

    let all_old_wallpapers_query = get_wallpapers();
    let all_old_wallpapers = all_old_wallpapers_query.unwrap_or_default();

    let mut old_wallpapers: Vec<Pair<&str, &str>> = vec![];
    let mut new_wallpapers: Vec<Pair<&str, &str>> = vec![];

    for section in wallpapers {
        let monitor_query = section.as_mapping_get("monitor");
        let wallpaper_query = section.as_mapping_get("wallpaper");

        if monitor_query.is_none() {
            return create_error("expected key monitor not found");
        }

        if wallpaper_query.is_none() {
            return create_error("expected key wallpaper not found");
        }

        let monitor_node = monitor_query.unwrap();
        let wallpaper_node = wallpaper_query.unwrap();

        if !monitor_node.is_string() {
            return create_error("expected key monitor can only be a string");
        }

        if !wallpaper_node.is_string() {
            return create_error(
                "expected key background can only be a string",
            );
        }

        let monitor = monitor_node.as_str().unwrap();
        let wallpaper = wallpaper_node.as_str().unwrap();

        new_wallpapers.push(Pair::new(monitor, wallpaper));

        if all_old_wallpapers.contains_key(monitor) {
            old_wallpapers
                .push(Pair::new(monitor, all_old_wallpapers[monitor].as_str()))
        }
    }

    for pair in new_wallpapers {
        let result = hyprctl::apply_wallpaper(pair.first(), pair.second());

        if let Err(error) = result {
            let mut error_message = error;
            let mut restoration_errors: Vec<String> = vec![];

            for pair in old_wallpapers.iter() {
                let maybe_failed =
                    hyprctl::apply_wallpaper(pair.first(), pair.second());

                if let Err(sub_error) = maybe_failed {
                    restoration_errors.push(sub_error);
                }
            }

            if restoration_errors.len() > 0 {
                error_message = format!(
                    "{}\nFurthermore, errors happened during restoration:\n{}",
                    error_message,
                    restoration_errors.join("\n")
                );
            }

            return Err(error_message);
        }
    }

    Ok(())
}
