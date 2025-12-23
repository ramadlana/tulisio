use regex::Regex;
use std::collections::HashSet;

pub fn extract_asset_paths(markdown: &str) -> HashSet<String> {
    let image_re = Regex::new(r"!\[[^\]]*\]\(([^)]+)\)").expect("valid image regex");
    let link_re = Regex::new(r"\[[^\]]+\]\(([^)]+)\)").expect("valid link regex");

    let mut paths = HashSet::new();

    for capture in image_re.captures_iter(markdown) {
        if let Some(path) = capture.get(1) {
            if let Some(normalized) = normalize_link_target(path.as_str()) {
                paths.insert(normalized);
            }
        }
    }

    for capture in link_re.captures_iter(markdown) {
        let full = capture.get(0).map(|m| m.as_str()).unwrap_or_default();
        let is_image = full.starts_with("![");
        if is_image {
            continue;
        }

        if let Some(path) = capture.get(1) {
            if let Some(normalized) = normalize_link_target(path.as_str()) {
                paths.insert(normalized);
            }
        }
    }

    paths
}

fn normalize_link_target(target: &str) -> Option<String> {
    let trimmed = target.trim().trim_matches('<').trim_matches('>');

    if trimmed.starts_with("http://")
        || trimmed.starts_with("https://")
        || trimmed.starts_with("mailto:")
    {
        return None;
    }

    Some(trimmed.replace('\\', "/"))
}
