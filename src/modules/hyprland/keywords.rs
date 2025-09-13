use saphyr::Yaml;

use crate::utils::{convert_yaml_scalar_to_string, Pair};

/// Append the keyword to the current namespace.
///
/// If there is no current namespace, the keyword is returned alone, thus
/// defining a new namespace.
///
/// # Arguments
///
/// * `current_namespace` - A string to place on the left side of the string.
/// * `new_keyword` - A string to place on right side of the string.
///
/// # Return
///
/// The result of the following format: `{current_namespace}:{new_keyword}`.
///
/// If `current_namespace` is empty, only `new_keyword` is returned as a new
/// String object.
fn append_keyword(current_namespace: &String, new_keyword: &str) -> String {
    if current_namespace.is_empty() {
        new_keyword.to_string()
    } else {
        format!("{}:{}", current_namespace, new_keyword)
    }
}

/// Parse the YAML hyprland subnode and generate the keywords to update.
///
/// As Hyprland uses section namespaces to organise all the different
/// updatable values, keywords can be produced in a recursive fashion. As
/// we choose to aggregate the keywords in a provided vector which is used to
/// to store all the results in one final place.
///
/// Keywords are stored as Pair tuples for better control on the data and to
/// keep the keyword and the value to set it to separates.
///
/// # Arguments
///
/// * `config` - A YAML configuration to parse.
/// * `prefix` - The prefix to apply to the next keywords. This is used to
///              store the namespace of the following keywords and is defined
///              internally for recursion. When used by a library user, it may
///              be simply set to an empty string.
/// * `result` - A vector of string pairs to store the result. This serves as
///              the return value.
pub fn generate_keywords(
    config: &Yaml<'_>,
    prefix: &String,
    result: &mut Vec<Pair<String, String>>,
) {
    let mapped_config = config.as_mapping().unwrap();

    for (key, value) in mapped_config {
        let string_key = key.as_str().unwrap();
        let sub_prefix = append_keyword(prefix, string_key);

        match value {
            Yaml::Mapping(_) => {
                generate_keywords(&value, &sub_prefix, result);
            }
            Yaml::Sequence(array) => {
                for sub_config in array {
                    generate_keywords(sub_config, &sub_prefix, result);
                }
            }
            Yaml::Value(value) => {
                let entry =
                    Pair::new(sub_prefix, convert_yaml_scalar_to_string(value));

                result.push(entry);
            }
            _ => unreachable!(),
        }
    }
}
