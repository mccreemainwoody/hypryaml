use std::process::{Command, Stdio};

use crate::utils::Pair;

fn fornat_keyword_definition(keyword: &Pair<String, String>) -> String {
    format!("keyword {} {}", keyword.first(), keyword.second())
}

pub fn apply_keywords_to_config(keywords: Vec<Pair<String, String>>) {
    let mut hyprctl = Command::new("hyprctl");

    let batch = keywords
        .iter()
        .map(fornat_keyword_definition)
        .collect::<Vec<String>>()
        .join(" ; ");

    let capture_mode = Stdio::piped;

    hyprctl
        .arg("--batch")
        .arg(batch)
        .stdout(capture_mode())
        .stderr(capture_mode())
        .status()
        .expect("hyprctl returned a non-zero exit code: ");
}
