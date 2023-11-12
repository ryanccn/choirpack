use std::fmt::Display;

use clap::ValueEnum;

#[derive(Clone, Debug, ValueEnum, PartialEq)]
pub enum PackageManager {
    Npm,

    Yarn,
    YarnClassic,

    Pnpm,
    Bun,
}

impl PackageManager {
    #[must_use]
    pub fn package_name(&self) -> String {
        match self {
            PackageManager::Npm => "npm".to_owned(),
            PackageManager::Yarn | PackageManager::YarnClassic => "yarn".to_owned(),
            PackageManager::Pnpm => "pnpm".to_owned(),
            PackageManager::Bun => "bun".to_owned(),
        }
    }

    #[must_use]
    pub fn with_version(&self, version: &str) -> String {
        self.package_name() + "@" + version
    }
}

impl Display for PackageManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            PackageManager::Npm => "npm".to_owned(),
            PackageManager::Yarn => "yarn".to_owned(),
            PackageManager::YarnClassic => "yarn (classic)".to_owned(),
            PackageManager::Pnpm => "pnpm".to_owned(),
            PackageManager::Bun => "bun".to_owned(),
        };

        write!(f, "{name}")
    }
}
