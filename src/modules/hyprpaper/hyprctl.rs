use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::process::Command;

use crate::utils::iter::all_same;
use crate::utils::path;
use crate::utils::system::extract_stdout;

/// Return a new command pointing to the `hyprctl` command.
///
/// # Return
///
/// A Command object with the base command set to `hyprctl`.
fn get_hyprctl() -> Command {
    Command::new("hyprctl")
}

/// Check that at most one unique value exists in `monitors`.
///
/// As the input is expected to be an iteration of monitor identifiers, the
/// `*` symbol is omitted from the evaluation.
///
/// # Parameters
///
/// * `monitors` - An `Iterator` of strings.
///
/// # Return
///
/// `true` if the iterators only contains one distinct value or no value after
/// removal of the `*` string. `false` otherwise.
fn only_one_monitor<'a>(monitors: impl Iterator<Item = &'a String>) -> bool {
    let mut all_monitors = monitors.map(|x| x.as_str()).collect::<HashSet<_>>();

    all_monitors.remove("*");

    return all_monitors.len() <= 1;
}

/// Extract the monitor and wallpaper
///
/// Assumes the input is a string referencing a monitor and its associated
/// wallpaper using the format `MONITOR: PATH`.
///
/// This is the format used by the output of `hyprctl hyprpaper listactive`,
/// which returns every monitor that has an active wallpaper.
///
/// # Parameters
///
/// * `line` - A string referencing a monitor and a wallpaper.
///
/// # Return
///
/// A tuple containing the monitor and the wallpaper in separate String objects.
fn extract_active_wallpaper(line: &str) -> (String, String) {
    let mut split_iter = line.split(": ");

    let first = split_iter.next();
    let second = split_iter.next();

    let first_str = first.unwrap_or("").to_string();
    let second_str = second.unwrap_or("").to_string();

    return (first_str, second_str);
}

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
    let mut hyprctl = get_hyprctl();

    let formatted_input =
        format!("{},{}", monitor, wallpaper.to_str().unwrap());

    let result = hyprctl
        .args(["hyprpaper", "wallpaper", formatted_input.as_str()])
        .output();

    match result {
        Ok(_) => Ok(()),
        Err(error) => Err(error.to_string()),
    }
}

/// Retrieve all currently active wallpapers from the hyprpaper daemon.
///
/// # Return
///
/// An `HashMap` referencing monitor identifiers as keys and wallpaper file
/// paths as values.
pub fn get_wallpapers() -> Result<HashMap<String, String>, String> {
    let mut hyprctl = get_hyprctl();

    let result = hyprctl.args(["hyprpaper", "listactive"]).output();

    if let Err(error) = result {
        return Err(error.to_string());
    }

    let mut current_config: HashMap<String, String> = HashMap::new();
    let stdout = extract_stdout(&result.unwrap())?;

    for line in stdout.lines() {
        let (monitor, wallpaper) = extract_active_wallpaper(line);

        current_config.insert(monitor, wallpaper);
    }

    if current_config.len() > 0
        && (all_same(&mut current_config.values())
            || only_one_monitor(current_config.keys()))
    {
        let monitor = "*".to_string();
        let wallpaper = current_config.values().next().unwrap().clone();

        return Ok(HashMap::from([(monitor, wallpaper)]));
    }

    Ok(current_config)
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
