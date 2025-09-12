use saphyr::Scalar;

pub fn convert_yaml_scalar_to_string(node: &Scalar<'_>) -> String {
    match node {
        Scalar::Boolean(bool) => format!("{}", bool),
        Scalar::FloatingPoint(float) => format!("{}", float),
        Scalar::Integer(int) => format!("{}", int),
        Scalar::String(string) => string.to_string(),
        _ => "null".to_string(),
    }
}
