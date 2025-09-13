use std::io;
use std::process::{Command, ExitStatus, Stdio};

use crate::utils::Pair;

/// Format the keyword pair to the string format used when calling hyprctl.
///
/// # Arguments
///
/// * `keyword` - A pair to format, containing the keyword as first value and
///               the value it must be set to as second value.
///
/// # Return
///
/// The formatted string : `keyword {&pair.0} {&pair.1}'.
///
/// This can be given as a set of parameter to hyprctl to update the correct
/// value.
fn format_keyword_definition(keyword: &Pair<String, String>) -> String {
    format!("keyword {} {}", keyword.first(), keyword.second())
}

/// Apply the specified list of keyword to the current Hyprland instance.
///
/// This processed is done using the hyprctl CLI as a middleware between
/// hypryaml and the Hyprland socket.
///
/// This process is run as a batch : settings are applied together or fail
/// together.
///
/// # Aguments
///
/// * `keywords` - A list of pairs. Each pair must contain the keyword to
///                update as their first value, and the value to set each
///                keyword to as second value.
///
/// # Return
///
/// A Result object. Ok contains an ExitStatus structure, while Error contains
/// the error specified as an io::Error object.
pub fn apply_keywords_to_config(
    keywords: Vec<Pair<String, String>>,
) -> Result<ExitStatus, io::Error> {
    let mut hyprctl = Command::new("hyprctl");

    let batch = keywords
        .iter()
        .map(format_keyword_definition)
        .collect::<Vec<String>>()
        .join(" ; ");

    let capture_mode = Stdio::piped;

    hyprctl
        .arg("--batch")
        .arg(batch)
        .stdout(capture_mode())
        .stderr(capture_mode())
        .status()
}
