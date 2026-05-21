use std::process::Command;

use crate::modules::notify::levels::NotifyLevel;

/// Send the notification to the notification daemon.
///
/// This uses the notify-send API in the background.
///
/// # Parameters
///
/// * `level` - The level to raise for the notification (Info, Error...).
/// * `content` - A string reference, which contains the content to write on
///               the notification.
/// * `timecout` - An integer representing the duration for which the
///                notification should stay on the user interface. Specified in
///                milliseconds.
///
/// # Return
///
/// A Result object. Ok contains nothing, while Error contains the error
/// specified as a String.
pub fn send(
    level: NotifyLevel,
    content: &str,
    timeout: &u32,
) -> Result<(), String> {
    let mut notify_send = Command::new("notify-send");
    let timeout_str = timeout.to_string();
    let urgency = match level {
        NotifyLevel::Info => "normal",
        NotifyLevel::Error => "critical",
    };

    let output = notify_send
        .args([
            "hypryaml",
            content,
            "--urgency",
            urgency,
            "--app-name",
            "hypryaml",
            "--expire-time",
            timeout_str.as_str(),
        ])
        .output();

    match output {
        Ok(_) => Ok(()),
        Err(error) => Err(error.to_string()),
    }
}
