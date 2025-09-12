use std::path::PathBuf;

pub fn validate_path(path: &PathBuf) -> Result<(), String> {
    let path_string = path.to_str().unwrap();

    if !path.exists() {
        let message = format!("the path at {} does not exist", path_string);

        Err(message)
    } else {
        Ok(())
    }
}
