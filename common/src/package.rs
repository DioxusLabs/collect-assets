use serde::{Deserialize, Serialize};

use crate::{
    asset::AssetType,
    cache::{current_package_cache_dir, current_package_identifier},
};

/// Clears all assets from the current package
pub fn clear_assets() -> std::io::Result<()> {
    let dir = current_package_cache_dir();
    if dir.exists() {
        tracing::info!("Clearing assets in {:?}", dir);
        std::fs::remove_dir_all(dir)?;
    }
    Ok(())
}

/// Adds an asset to the current package
pub fn add_asset(asset: AssetType) -> std::io::Result<AssetType> {
    let mut dir = current_package_cache_dir();
    dir.push("assets.toml");
    tracing::info!("Adding asset {:?} to {:?}", asset, dir);
    let mut package_assets: PackageAssets = if dir.exists() {
        let contents = std::fs::read_to_string(&dir)?;
        toml::from_str(&contents).unwrap_or_else(|_| PackageAssets {
            package: current_package_identifier(),
            assets: vec![],
        })
    } else {
        std::fs::create_dir_all(dir.parent().unwrap())?;
        PackageAssets {
            package: current_package_identifier(),
            assets: vec![],
        }
    };

    package_assets.add(asset.clone());
    let contents = toml::to_string(&package_assets).unwrap();
    std::fs::write(&dir, contents)?;

    Ok(asset)
}

/// All assets collected from a specific package
#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub struct PackageAssets {
    package: String,
    assets: Vec<AssetType>,
}

impl PackageAssets {
    /// Adds an asset to the package
    pub fn add(&mut self, asset: AssetType) {
        self.assets.push(asset);
    }

    /// Returns a reference to the package name
    pub fn package(&self) -> &str {
        &self.package
    }

    /// Returns a reference to the assets in this package
    pub fn assets(&self) -> &Vec<AssetType> {
        &self.assets
    }
}
