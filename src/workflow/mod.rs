use std::path::PathBuf;

pub fn apply_configuration(raw_config: &PathBuf) -> Result<(), &str>{
    println!("{}", raw_config.display());

    Ok(())
}
