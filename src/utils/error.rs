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
pub fn create_error(message: &str) -> Result<(), String> {
    Err(message.to_string())
}
