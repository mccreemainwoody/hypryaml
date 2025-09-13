use saphyr::Scalar;

/// Convert any YAML scalar value to its String representation.
///
/// Numerical and binary values are converted to string using the `format`
/// macro.
///
/// If the scalar is neither a boolean, a floating value, an integer or a
/// string, the function returns "null" instead.
///
/// # Parameters
///
/// * `node` - The YAML Scalar node to extract from.
///
/// # Return
///
/// Whatever the node contains, formatted into a new String.
pub fn convert_yaml_scalar_to_string(node: &Scalar<'_>) -> String {
    match node {
        Scalar::Boolean(bool) => format!("{}", bool),
        Scalar::FloatingPoint(float) => format!("{}", float),
        Scalar::Integer(int) => format!("{}", int),
        Scalar::String(string) => string.to_string(),
        _ => "null".to_string(),
    }
}
