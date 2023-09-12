use anyhow::{anyhow, Result};
use tokio::fs;

use regex::Regex;
use serde::Deserialize;

use std::path::PathBuf;

use crate::PackageManager;

mod whitespace;

#[derive(Debug, Deserialize)]
struct PackageJson {
    #[serde(rename = "packageManager", skip_serializing_if = "Option::is_none")]
    package_manager: Option<String>,
}

pub async fn get_package_manager(path: &PathBuf) -> Result<PackageManager> {
    let text = fs::read_to_string(path).await?;
    let data = serde_json::from_str::<PackageJson>(&text)?;
    let package_manager_str = data
        .package_manager
        .ok_or_else(|| anyhow!("Could not find the packageManager key"))?;

    if package_manager_str.starts_with("npm@") {
        Ok(PackageManager::Npm)
    } else if package_manager_str.starts_with("yarn@1") {
        Ok(PackageManager::YarnClassic)
    } else if package_manager_str.starts_with("yarn@") {
        Ok(PackageManager::Yarn)
    } else if package_manager_str.starts_with("pnpm@") {
        Ok(PackageManager::Pnpm)
    } else {
        Err(anyhow!("Could not find a valid packageManager value"))
    }
}

pub async fn patch_package_manager(
    path: &PathBuf,
    package_manager: &PackageManager,
    version: &str,
) -> Result<()> {
    let text = fs::read_to_string(path).await?;

    let existing_regex = Regex::new("\"packageManager\": \"[A-Za-z]+@[\\dA-Za-z\\.]+\"")?;

    if existing_regex.is_match(&text) {
        let new_text = existing_regex.replace(
            &text,
            format!(
                "\"packageManager\": \"{}@{}\"",
                package_manager.to_package_name(),
                version
            ),
        );
        fs::write(path, new_text.to_string()).await?;
    } else {
        let mut lines = text
            .split('\n')
            .map(|s| s.to_owned())
            .collect::<Vec<String>>();

        let whitespace_str = whitespace::determine(&text)?.to_string();
        let new_line = format!(
            "{}\"packageManager\": \"{}@{}\"",
            whitespace_str,
            package_manager.to_package_name(),
            version
        );

        let last_meaningful_line_idx = lines.len() - 3;

        let replace_line = lines.get_mut(last_meaningful_line_idx).unwrap();
        if *replace_line != "{" && !replace_line.ends_with(',') {
            *replace_line = replace_line.to_owned() + ",";
        }

        lines.insert(lines.len() - 2, new_line);

        let new_text = lines.join("\n");
        fs::write(path, new_text).await?;
    }

    Ok(())
}
