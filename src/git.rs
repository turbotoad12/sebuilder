use git2::Repository;
use std::path::Path;

pub fn clone_tag(
    repo_url: &str,
    tag: Option<&str>,
    dest: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Determine which tag to use
    let tag = match tag {
        Some(t) if !t.trim().is_empty() => t.to_string(),
        _ => {
            println!("No tag provided — fetching latest tag...");
            crate::git::get_latest_se_tag()? // call your sync function
        }
    };

    println!("Cloning tag: {}", tag);

    // Clone the repo
    let repo = Repository::clone(repo_url, Path::new(dest))?;

    // Resolve tag reference
    let tag_ref = format!("refs/tags/{}", tag);
    let reference = repo.find_reference(&tag_ref)?;

    // Peel to commit
    let commit = reference.peel_to_commit()?;

    // Checkout the commit
    repo.checkout_tree(commit.as_object(), None)?;
    repo.set_head_detached(commit.id())?;

    Ok(())
}


use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use semver::Version;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Tag {
    name: String,
}
#[allow(clippy::collapsible_if)]
fn normalize(tag: &str) -> Option<Version> {
    // Already valid semver?
    if let Ok(v) = Version::parse(tag) {
        return Some(v);
    }

    // Handle X.Y → X.Y.0
    if let Some((major, minor)) = tag.split_once('.') {
        if minor.chars().all(|c| c.is_ascii_digit()) {
            let fixed = format!("{}.{}.0", major, minor);
            return Version::parse(&fixed).ok();
        }
    }

    // Handle X.Y-rcZ → X.Y.0-rcZ
    if let Some((base, rc)) = tag.split_once("-rc") {
        if let Some((major, minor)) = base.split_once('.') {
            if minor.chars().all(|c| c.is_ascii_digit()) {
                let fixed = format!("{}.{}.0-rc{}", major, minor, rc);
                return Version::parse(&fixed).ok();
            }
        }
    }

    None
}

pub fn get_latest_se_tag() -> Result<String, Box<dyn std::error::Error>> {
    let url = "https://api.github.com/repos/ScratchEverywhere/ScratchEverywhere/tags";

    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("visual-scratch-updater"));

    let client = Client::new();
    let tags: Vec<Tag> = client.get(url).headers(headers).send()?.json()?;

    let mut versions: Vec<(String, Version)> = tags
        .iter()
        .filter_map(|t| normalize(&t.name).map(|v| (t.name.clone(), v)))
        .collect();

    if versions.is_empty() {
        return Err("No valid tags found".into());
    }

    // Sort by semver descending
    versions.sort_by(|a, b| b.1.cmp(&a.1));

    // Return the original tag string, not the normalized one
    Ok(versions[0].0.clone())
}


// -------- TESTS --------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_valid_semver() {
        let v = normalize("0.39.1").unwrap();
        assert_eq!(v.to_string(), "0.39.1");
    }

    #[test]
    fn test_normalize_missing_patch() {
        let v = normalize("0.40").unwrap();
        assert_eq!(v.to_string(), "0.40.0");
    }

    #[test]
    fn test_normalize_rc_version() {
        let v = normalize("1.0-rc3").unwrap();
        assert_eq!(v.to_string(), "1.0.0-rc3");
    }

    #[test]
    fn test_sorting_latest() {
        let tags = vec![
            "0.39.1",
            "0.40",
            "1.0-rc1",
            "1.0-rc3",
            "1.0-rc2",
        ];

        let mut versions: Vec<(String, semver::Version)> = tags
            .iter()
            .filter_map(|t| normalize(t).map(|v| (t.to_string(), v)))
            .collect();

        versions.sort_by(|a, b| b.1.cmp(&a.1));

        assert_eq!(versions[0].0, "1.0-rc3");
    }
}


