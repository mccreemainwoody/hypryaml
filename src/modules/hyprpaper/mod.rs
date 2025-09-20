use saphyr::{Sequence, Yaml};

use crate::utils::error::throw_error;

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
        return throw_error("hyprpaper configuration should be a sequence");
    }

    let wallpapers = config.as_sequence().unwrap();

    if !validate_config_sequence(wallpapers) {
        return throw_error(
            "hyprpaper configuration must be a sequence of mappings",
        );
    }

    for section in wallpapers {
        let monitor_query = section.as_mapping_get("monitor");
        let wallpaper_query = section.as_mapping_get("wallpaper");

        if monitor_query.is_none() {
            return throw_error("expected key monitor not found");
        }

        if wallpaper_query.is_none() {
            return throw_error("expected key wallpaper not found");
        }

        let monitor = monitor_query.unwrap();
        let wallpaper = wallpaper_query.unwrap();

        if !monitor.is_string() {
            return throw_error("expected key monitor can only be a string");
        }

        if !wallpaper.is_string() {
            return throw_error("expected key background can only be a string");
        }

        let monitor_str = monitor.as_str().unwrap();
        let background_str = wallpaper.as_str().unwrap();

        hyprctl::apply_wallpaper(&monitor_str, &background_str)?
    }

    Ok(())
}
