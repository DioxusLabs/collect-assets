//! Utilities for the cache that is used to collect assets

use std::{
    fmt::{Display, Write},
    path::PathBuf,
};

use home::cargo_home;

/// The location where assets are cached
pub fn asset_cache_dir() -> PathBuf {
    let mut dir = cargo_home().unwrap();
    dir.push("assets");
    dir
}

pub(crate) fn config_path() -> PathBuf {
    asset_cache_dir().join("config.toml")
}

pub(crate) fn current_package_identifier() -> String {
    package_identifier(
        &std::env::var("CARGO_PKG_NAME").unwrap(),
        std::env::var("CARGO_BIN_NAME").ok().as_deref(),
        &current_package_version(),
    )
}

/// The identifier for a package used to cache assets
pub fn package_identifier(package: &str, bin: 
    Option<&str>
    , version: &str) -> String {
    let mut string = package.to_string();
    if let Some(bin) = bin {
        string.push('-');
        string.push_str(bin);
    }
    string.push('-');
    string.push_str(version);
    string
}

/// Like `package_identifier`, but appends the identifier to the given path
pub fn push_package_cache_dir(package: &str, 
    bin: Option<&str>,
    version: impl Display, dir: &mut PathBuf) {
    let as_string = dir.as_mut_os_string();
    as_string.write_char(std::path::MAIN_SEPARATOR).unwrap();
    as_string.write_str(package).unwrap();
    if let Some(bin) = bin {
        as_string.write_char('-').unwrap();
        as_string.write_str(bin).unwrap();
    }
    as_string.write_char('-').unwrap();
    as_string.write_fmt(format_args!("{}", version)).unwrap();
}

pub(crate) fn current_package_version() -> String {
    std::env::var("CARGO_PKG_VERSION").unwrap()
}

pub(crate) fn manifest_dir() -> PathBuf {
    std::env::var("CARGO_MANIFEST_DIR").unwrap().into()
}

pub(crate) fn current_package_cache_dir() -> PathBuf {
    let mut dir = asset_cache_dir();
    dir.push(current_package_identifier());
    dir
}
