use std::collections::HashMap;

use anyhow::{anyhow, Result};
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::PackageManager;

#[derive(Serialize, Deserialize, Debug)]
struct NpmRegistryResponse {
    #[serde(rename = "dist-tags")]
    dist_tags: HashMap<String, String>,
}

pub async fn fetch_latest(package_manager: &PackageManager) -> Result<String> {
    let resp = reqwest::get(format!(
        "https://registry.npmjs.com/{}",
        package_manager.to_package_name()
    ))
    .await?;

    let data: NpmRegistryResponse = resp.json().await?;

    let latest_tag = data.dist_tags.get(match package_manager {
        PackageManager::Yarn => "berry",
        _ => "latest",
    });

    match latest_tag {
        Some(latest_tag) => Ok(latest_tag.to_owned()),
        None => Err(anyhow!(
            "Could not find latest version of {}",
            package_manager
        )),
    }
}

pub fn version_value_parser(str: &str) -> Result<String> {
    let validator = Regex::new("[\\d.]+")?;

    if !validator.is_match(str) {
        return Err(anyhow!("not a valid version"));
    }

    Ok(str.to_owned())
}
