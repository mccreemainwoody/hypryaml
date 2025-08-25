use std::path::PathBuf;


pub fn validate_path(path: &PathBuf) -> Result<(), String> {
    let path_string = path.to_str().unwrap();

    if !path.exists() {
        let message = format!(
            "{} {} {}",
            "the path at",
            path_string,
            "does not exist"
        );

        return Err(message);
    }

    Ok(())
}
