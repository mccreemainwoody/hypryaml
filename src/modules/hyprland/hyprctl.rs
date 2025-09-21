use std::process::{Command, Stdio};

use crate::utils::{system::extract_stdout, Pair};

/// Clean the text from unwanted characters.
///
/// For now, it just merges all backslashes into a same series as one.
///
/// # Parameters
///
/// * `string` - The string to clean.
///
/// # Return
///
/// A new String representing the cleansed value.
fn clean_text(string: &String) -> String {
    let mut result = string.clone();

    while result.find("\n\n").is_some() {
        result = result.replace("\n\n", "\n");
    }

    result
}

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
/// # Aguments
///
/// * `keywords` - A list of pairs. Each pair must contain the keyword to
///                update as their first value, and the value to set each
///                keyword to as second value.
///
/// # Return
///
/// A Result object. Ok contains nothing, while Error contains the error
/// specified as a String.
pub fn apply_keywords_to_config(
    keywords: Vec<Pair<String, String>>,
) -> Result<(), String> {
    let mut hyprctl = Command::new("hyprctl");

    let batch = keywords
        .iter()
        .map(format_keyword_definition)
        .collect::<Vec<String>>()
        .join(" ; ");

    let capture_mode = Stdio::piped;

    let result = hyprctl
        .arg("--batch")
        .arg(batch)
        .stdout(capture_mode())
        .stderr(capture_mode())
        .output();

    match result {
        Ok(output) => {
            let stdout = extract_stdout(&output)?;
            let cleaned_stdout = clean_text(&stdout);

            let good_line = "ok";
            let erronous_lines: Vec<&str> = cleaned_stdout
                .split("\n")
                .into_iter()
                .filter(|line| !line.is_empty() && line != &good_line)
                .collect();

            match erronous_lines.len() {
                0 => Ok(()),
                _ => Err(erronous_lines.join("\n")),
            }
        }
        Err(error) => Err(error.to_string()),
    }
}
