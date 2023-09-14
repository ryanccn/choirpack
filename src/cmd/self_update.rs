use anyhow::{anyhow, Result};
use async_trait::async_trait;

use indicatif::{ProgressBar, ProgressStyle};
use owo_colors::OwoColorize;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::Deserialize;

use clap::Args;
use futures_util::StreamExt;
use tokio::{fs::File, io::AsyncWriteExt};

#[derive(Args, Debug)]
pub struct Options {}

#[derive(Deserialize, Clone, Debug)]
struct GitHubReleaseAsset {
    name: String,
    browser_download_url: String,
}

#[derive(Deserialize, Clone, Debug)]
struct GitHubRelease {
    tag_name: String,
    assets: Vec<GitHubReleaseAsset>,
}

fn get_artifact_name() -> Result<String> {
    use std::env::consts::{ARCH, OS};

    let err = anyhow!("No GitHub Releases build available for {} {}", OS, ARCH);

    match OS {
        "macos" => match ARCH {
            "aarch64" => Ok("choirpack-aarch64-apple-darwin".to_owned()),
            "x86_64" => Ok("choirpack-x86_64-apple-darwin".to_owned()),
            &_ => Err(err),
        },
        "linux" => match ARCH {
            "aarch64" => Ok("choirpack-aarch64-unknown-linux-musl-static".to_owned()),
            "x86_64" => Ok("choirpack-x86_64-unknown-linux-musl-static".to_owned()),
            &_ => Err(err),
        },
        "windows" => match ARCH {
            "x86_64" => Ok("choirpack-x86_64-pc-windows-msvc.exe".to_owned()),
            &_ => Err(err),
        },
        &_ => Err(err),
    }
}

#[async_trait]
impl super::OptionsWithAction for Options {
    async fn action(&self) -> Result<()> {
        let mut default_headers = HeaderMap::new();
        default_headers.insert("user-agent", HeaderValue::from_static("choirpack/"));

        let http = reqwest::Client::builder()
            .default_headers(default_headers)
            .build()?;

        let resp = http
            .get("https://api.github.com/repos/ryanccn/choirpack/releases")
            .send()
            .await?;
        resp.error_for_status_ref()?;

        let data: Vec<GitHubRelease> = resp.json().await?;
        let latest = data
            .first()
            .ok_or_else(|| anyhow!("Could not obtain latest version"))?;

        let artifact_name = get_artifact_name()?;

        let artifact = latest
            .assets
            .iter()
            .find(|asset| asset.name == artifact_name)
            .ok_or_else(|| anyhow!("Could not find latest version artifact to download"))?;

        let resp = http.get(&artifact.browser_download_url).send().await?;
        resp.error_for_status_ref()?;

        let total_size = resp
            .content_length()
            .ok_or(anyhow!("Failed to get content length"))?;

        let pb = ProgressBar::new(total_size);

        pb.set_style(
            ProgressStyle::default_bar()
                .template(
                    "{msg}\n{bar:40.cyan/dim} {bytes}/{total_bytes} ({bytes_per_sec}, {eta})",
                )?
                .progress_chars("━━━"),
        );

        pb.set_message(format!("Downloading {}", latest.tag_name.green()));

        let self_path = std::env::current_exe()?;
        let mut self_file = File::create(self_path).await?;

        let mut stream = resp.bytes_stream();
        let mut downloaded: u64 = 0;

        while let Some(Ok(chunk)) = stream.next().await {
            self_file.write_all(&chunk).await?;

            let new = std::cmp::min(downloaded + (chunk.len() as u64), total_size);
            downloaded = new;
            pb.set_position(new);
        }

        pb.finish_with_message(format!("Updated to {}", latest.tag_name.green()));

        Ok(())
    }
}
