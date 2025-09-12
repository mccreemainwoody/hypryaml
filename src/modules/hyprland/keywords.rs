use saphyr::Yaml;

use crate::utils::{convert_yaml_scalar_to_string, Pair};

fn append_keyword(current_namespace: &String, new_keyword: &str) -> String {
    if current_namespace.is_empty() {
        new_keyword.to_string()
    } else {
        format!("{}:{}", current_namespace, new_keyword)
    }
}

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
