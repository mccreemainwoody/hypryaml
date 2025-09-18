use saphyr::{Sequence, Yaml};

mod hyprctl;

/// Utility function to create an Err object based on an static string
/// smoothly.
///
/// # Arguments
///
/// * `message` - The message to return with the error as a static string (or
///               any other type of string reference).
///
/// # Return
///
/// A new Err object containing the specified message as a String object.
fn throw_error(message: &str) -> Result<(), String> {
    Err(message.to_string())
}

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
/// hyprland:
///   - monitor: DPD-1
///     background: /home/me/amogus.jpg
///   - monitor: DPD-2
///     background: /home/me/sugoma.jpg
/// ```
pub fn apply_config(config: &Yaml<'_>) -> Result<(), String> {
    if !config.is_sequence() {
        return throw_error(
            "hyprpaper configuration should be a suite of subnodes",
        );
    }

    let wallpapers = config.as_sequence().unwrap();

    if !validate_config_sequence(wallpapers) {
        return throw_error("hyprpaper configuration should only be mapping");
    }

    for section in wallpapers {
        let monitor_query = section.as_mapping_get("monitor");
        let background_query = section.as_mapping_get("background");

        if monitor_query.is_none() {
            return throw_error("expected key monitor not found");
        }

        if background_query.is_none() {
            return throw_error("expected key background not found");
        }

        let monitor = monitor_query.unwrap();
        let background = background_query.unwrap();

        if !monitor.is_string() {
            return throw_error("expected key monitor can only be a string");
        }

        if !background.is_string() {
            return throw_error("expected key background can only be a string");
        }

        let monitor_str = monitor.clone().into_string().unwrap();
        let background_str = background.clone().into_string().unwrap();

        let result = hyprctl::apply_wallpaper(&monitor_str, &background_str);

        if result.is_err() {
            let error = result.err().unwrap();

            return Err(error);
        }
    }

    Ok(())
}
