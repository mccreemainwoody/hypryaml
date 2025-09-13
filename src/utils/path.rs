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
