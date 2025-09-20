use std::process::Output;

/// Extract the STDOUT content from the Output and return it as a String.
///
/// # Parameters
///
/// * `output` - An Output object to extract from.
///
/// # Return
///
/// A Result object. Ok contains the decoded content of standard output as a
/// String object, while Error contains a wrapped version of the conversion
/// error.
pub fn extract_stdout(output: &Output) -> Result<String, String> {
    let raw_stdout = &output.stdout;

    let extraction_attempt = String::from_utf8(raw_stdout.clone());

    match extraction_attempt {
        Ok(string) => Ok(string),
        Err(reason) => Err(format!(
            "failure during stdout conversion :\n{}",
            reason.to_string()
        )),
    }
}
