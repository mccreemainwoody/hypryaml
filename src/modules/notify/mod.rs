mod levels;
mod send_notify;

use levels::NotifyLevel;

/// Send a notification matching the result stored at content.
///
/// The type is automatically deduced based on the type of `content` (`Info`
/// for `Ok`, `Error`for `Err`).
///
/// # Parameters
///
/// * `content` - The Result object to represent as a notification.
/// * `timecout` - An integer representing the duration for which the
///                notification should stay on the user interface. Specified in
///                milliseconds.
pub fn send_notification(content: &Result<&str, &str>, timeout: &u32) {
    let (level, content) = match content {
        Ok(message) => (NotifyLevel::Info, message),
        Err(reason) => (NotifyLevel::Error, reason),
    };

    let result = send_notify::send(level, content, timeout);

    match result {
        Err(reason) => eprintln!(
            "warning: failed to send notification. reason: {}",
            reason
        ),
        _ => {}
    }
}
