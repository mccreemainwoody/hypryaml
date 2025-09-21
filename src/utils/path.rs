use shellexpand;
use std::path::Path;
use std::path::PathBuf;

/// Throw an erronous result if the path does not exist.
///
/// # Arguments
///
/// * path - Any path one wishes to validate.
///
/// # Return
///
/// A Result object. Ok contains nohing while Error contains the error message
/// stored as a String.
pub fn validate_path(path: &PathBuf) -> Result<(), String> {
    let path_string = path.to_str().unwrap();

    if !path.exists() {
        let message = format!("the path at {} does not exist", path_string);

        Err(message)
    } else {
        Ok(())
    }
}

/// Interpret the string Path as an existing Path object and return it.
///
/// Will return `None` if the path does not exist.
///
/// # Arguments
///
/// * `filepath` - Any path, represented as a string.
///
/// # Return
///
/// An Option object. If it does contain something, then the returned value is
/// the path string represented as a new Path object.
pub fn deduce_path(filepath: &str) -> Option<&Path> {
    let maybe_path = Path::new(filepath);

    if !maybe_path.exists() {
        None
    } else {
        Some(maybe_path)
    }
}

/// Expand the provided path and return a new String object.
///
/// Environment variables and home path are extended through this method.
///
/// # Arguments
///
/// * `path` - Any valid path, represented as a string.
///
/// # Return
///
/// A Result Object. Ok contains the extended Path while Error contains the
/// reason of the extension failure.
pub fn expand_path(path: &str) -> Result<String, String> {
    let expanded_string_attempt = shellexpand::full(path);

    match expanded_string_attempt {
        Ok(expanded_string) => Ok(expanded_string.to_string()),
        Err(reason) => {
            Err(format!("Failed to expand file path {} :\n{}", path, reason))
        }
    }
}
