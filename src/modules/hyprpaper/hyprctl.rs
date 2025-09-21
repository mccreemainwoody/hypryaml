use std::path::Path;
use std::process::{Command, Stdio};

use crate::utils::path;
use crate::utils::system::extract_stdout;

/// Load the wallpaper, running the corresponding call to `hyprpaper reload`.
///
/// # Parameters
///
/// * `monitor_name` - A string reference, which defines the monitor that must
///                    receive the wallpaper update. The value * will be
///                    translated to the hyprpaper wildcard empty string token.
/// * `wallpaper` - A Path object, which references the path to the wallpaper
///                 to apply. It MUST be conform to hyprpaper standards (be an
///                 absolute path, be valid).
///
/// # Return
///
/// A Result object. Ok contains nothing, while Error contains the error
/// specified as a String.
fn load_wallpaper(monitor: &str, wallpaper: &Path) -> Result<(), String> {
    let mut hyprctl = Command::new("hyprctl");
    let capture_mode = Stdio::piped;

    let formatted_input =
        format!("{},{}", monitor, wallpaper.to_str().unwrap());

    let result = hyprctl
        .args(["hyprpaper", "reload", formatted_input.as_str()])
        .stdout(capture_mode())
        .stdout(capture_mode())
        .output();

    match result {
        Ok(output) => {
            let stdout = extract_stdout(&output)?;

            let cleaned_stdout = stdout.trim();

            match cleaned_stdout {
                "ok" => Ok(()),
                _ => Err(cleaned_stdout.to_string()),
            }
        }
        Err(error) => Err(error.to_string()),
    }
}

/// Apply the wallpaper over the specified monitor.
///
/// The wallpaper MUST reference a valid path to be able to run the hyprpaper
/// command. If it isn't, the function will return an error on call. The method
/// takes care to expand the path in a way similar to bash if necessary (HOME,
/// environment variables...). If it can't, the method will also return an
/// error.
///
/// To match the hyprpaper API, monitor values which doesn't reference any
/// monitor detected by Hyprland will not lead to an error and will be applied
/// within hyprpaper.
///
/// This processed is done using the hyprctl CLI as a middleware between
/// hypryaml and the Hyprpaper socket.
///
/// # Arguments
///
/// * `monitor_name` - A string reference, which defines the monitor that must
///                    receive the wallpaper update. The value * will be
///                    translated to the hyprpaper wildcard empty string token.
/// * `wallpaper_path` - A string reference, which references the path of the
///                      wallpaper. It can use bash-expanded values, like the
///                      HOME tilde and environment variables.
///
/// # Return
///
/// A Result object. Ok contains nothing, while Error contains the error
/// specified as a String.
pub fn apply_wallpaper(
    monitor_name: &str,
    wallpaper_path: &str,
) -> Result<(), String> {
    let monitor = match monitor_name {
        "*" => "",
        _ => monitor_name,
    };

    let expanded_wallpaper_path = path::expand_path(wallpaper_path)?;
    let wallpaper_search = path::deduce_path(&expanded_wallpaper_path);

    match wallpaper_search {
        Some(wallpaper) => load_wallpaper(monitor, wallpaper),
        None => Err(format!("file {} does not exist", wallpaper_path)),
    }
}
