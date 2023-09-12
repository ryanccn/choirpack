use std::time::Duration;

use anyhow::Result;
use owo_colors::OwoColorize;
use tokio::fs;

use crate::corepack;

async fn clean_package_manager(name: &str, duration: &Duration) -> Result<()> {
    let folder = corepack::get_folder()?;

    let inner_folder = folder.join(name);
    let mut contents = fs::read_dir(&inner_folder).await?;

    let mut old_versions = Vec::<String>::new();

    while let Some(v) = contents.next_entry().await? {
        let is_dir = v.file_type().await?.is_dir();

        if is_dir {
            let metadata = v.metadata().await?;
            let age = metadata.accessed()?.elapsed()?;

            if age > *duration {
                old_versions.push(v.file_name().to_string_lossy().to_string());
            }
        }
    }

    for old_version in &old_versions {
        fs::remove_dir_all(inner_folder.join(old_version)).await?;
    }

    println!(
        "{} {} version{} of {}",
        "Removed".green(),
        old_versions.len().bold(),
        if old_versions.len() == 1 { "" } else { "s" },
        name
    );

    if !&old_versions.is_empty() {
        println!("{}", old_versions.join(", ").dimmed());
    }

    Ok(())
}

pub struct Options {
    pub duration: String,
}

pub async fn action(options: Options) -> Result<()> {
    let duration = humantime::parse_duration(&options.duration)?;
    let formatted_duration = humantime::format_duration(duration);

    println!(
        "Removing versions unused for {}...",
        format!("more than {}", formatted_duration).yellow()
    );

    clean_package_manager("pnpm", &duration).await?;
    clean_package_manager("yarn", &duration).await?;
    clean_package_manager("npm", &duration).await?;

    Ok(())
}
