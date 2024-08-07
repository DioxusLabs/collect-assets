use std::{
    fmt::Display,
    hash::{DefaultHasher, Hash, Hasher},
    path::{Path, PathBuf},
    str::FromStr,
};

use anyhow::Context;
use base64::Engine;
use http::Uri;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{config, AssetError, FileOptions};

/// An asset identified by a URI
///
/// This could be a file, a folder, a remote URL, a data-encoded string, etc.
///
/// We don't want to download or copy the resource itself, just the metadata about it such that
/// we can resolve it later.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Hash, Eq)]
pub struct ResourceAsset {
    /// The input URI
    ///
    /// This is basically whatever the user passed in to the macro
    #[serde(with = "http_serde::uri")]
    pub input: Uri,

    /// The local URI for fallbacks
    ///
    /// This generally retains the original URI that was used to resolve the asset, but for files,
    /// it's resolved to an absolute path since we transform all schema-less URIs to file:// URIs.
    #[serde(with = "http_serde::uri")]
    pub local: Uri,

    /// The output URI that makes it into the final bundle.
    /// This explicitly has the `bundle://` scheme to make it clear that it is a bundle URI.
    ///
    /// The bundler will generate a unique name for the asset and use that as the path to generate a
    /// final "flat" architecture.
    ///
    /// bundle://asset/path/to/file.txt
    #[serde(with = "http_serde::uri")]
    pub bundled: Uri,

    /// The options for the resource
    pub options: Option<FileOptions>,
}

impl ResourceAsset {
    ///
    pub fn new(raw: &str) -> Self {
        todo!()
    }

    ///
    pub fn unique_name(&self) -> &str {
        todo!()
    }

    ///
    pub fn original(&self) -> &Uri {
        todo!()
    }

    /// Set the file options
    pub fn with_options(self, options: FileOptions) -> Self {
        todo!()
        // let mut myself = Self {
        //     options,
        //     url_encoded: false,
        //     ..self
        // };

        // myself.regenerate_unique_name();

        // myself
    }

    ///
    pub fn set_options(&mut self, options: FileOptions) {
        self.options = Some(options);
    }

    /// Set whether the file asset should be url encoded
    pub fn set_url_encoded(&mut self, url_encoded: bool) {
        todo!()
        // self.url_encoded = url_encoded;
    }

    /// Returns whether the file asset should be url encoded
    pub fn url_encoded(&self) -> bool {
        todo!()
        // self.url_encoded
    }

    /// Parse a string as a file source
    pub fn parse_file(path: &str) -> Result<Self, AssetError> {
        // let myself = Self::parse_any(path)?;
        // if let Self::Local(path) = &myself {
        //     if !path.canonicalized.is_file() {
        //         return Err(AssetError::NotFile(path.canonicalized.to_path_buf()));
        //     }
        // }
        // Ok(myself)
        todo!()
    }

    ///
    pub fn as_url(&self) -> Option<&Url> {
        todo!()
    }

    /// Parse a string as a folder source
    pub fn parse_folder(path: &str) -> Result<Self, AssetError> {
        // let myself = Self::parse_any(path)?;
        // if let Self::Local(path) = &myself {
        //     if !path.canonicalized.is_dir() {
        //         return Err(AssetError::NotFolder(path.canonicalized.to_path_buf()));
        //     }
        // }
        // Ok(myself)
        todo!()
    }

    ///
    pub fn parse_url(url: &str) -> Result<Self, AssetError> {
        todo!()
    }

    ///
    pub fn from_url(url: Url) -> Result<Self, AssetError> {
        todo!()
    }

    /// Parse a string as a file or folder source
    pub fn parse_any(src: &str) -> Result<Self, AssetError> {
        // todo!()
        // // Attempt to parse every path as a url - kinda dumb but it works
        // if let Ok(maybe_url) = Url::parse(src) {
        //     return Self::from_url(maybe_url);
        // }

        // Process the input as a URI
        let input: Uri = src.parse().unwrap();

        // Join the URI against the filesystem
        let manifest_dir: PathBuf = std::env::var("CARGO_MANIFEST_DIR").unwrap().into();
        let manifest_dir = manifest_dir.canonicalize().unwrap();
        // let manifest_dir = Uri::from_str(manifest_dir.to_str().unwrap()).unwrap();
        let local = manifest_dir.join(input.path());
        let local = Uri::from_str(local.to_str().unwrap()).unwrap();

        // And then generate the hash we need the bundled
        let bundled = local.clone();

        Ok(Self {
            input,
            local,
            bundled,
            options: None,
        })

        // // Paths are always relative to the manifest directory.
        // // If the path is absolute, we need to make it relative to the manifest directory.
        // let path = path
        //     .strip_prefix(std::path::MAIN_SEPARATOR_STR)
        //     .unwrap_or(&path);

        // let relative = PathBuf::from(std::path::MAIN_SEPARATOR_STR).join(path);

        // let path = manifest_dir.join(path);

        // match path.canonicalize() {
        //     Ok(x) => Ok(ResourceAsset {
        //         original: path.clone(),
        //         relative,
        //         canonicalized: x,
        //         url: None,
        //         unique_name: Default::default(),
        //         options: todo!(),
        //         url_encoded: todo!(),
        //     }),

        //     // relative path does not exist
        //     Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
        //         Err(AssetError::NotFoundRelative(manifest_dir, src.into()))
        //     }

        //     // other error
        //     Err(e) => Err(AssetError::IO(path, e)),
        // }
    }

    ///
    pub fn is_dir(&self) -> bool {
        todo!()
    }

    ///
    pub fn resolve(&self) -> String {
        // fn resolve_asset_location(location: &AssetSource) -> Result<String, ManganisSupportError> {
        //     if !config::is_bundled() {
        //         return Ok(location.source().raw());
        //     }

        //     let root = crate::config::base_path();
        //     let path = root.join(location.unique_name());
        //     Ok(path.display().to_string())
        // }

        todo!()
    }

    ///
    pub fn normalized(&self, extension: Option<&str>) -> String {
        // /// Create a normalized file name from the source
        // fn normalized_file_name(location: &AssetSource, extension: Option<&str>) -> String {
        //     let last_segment = location.last_segment();
        //     let mut file_name = to_alphanumeric_string_lossy(last_segment);

        //     let extension_len = extension.map(|e| e.len() + 1).unwrap_or_default();
        //     let extension_and_hash_size = extension_len + HASH_SIZE;
        //     // If the file name is too long, we need to truncate it
        //     if file_name.len() + extension_and_hash_size > MAX_PATH_LENGTH {
        //         file_name = file_name[..MAX_PATH_LENGTH - extension_and_hash_size].to_string();
        //     }
        //     file_name
        // }

        // /// Normalize a string to only contain alphanumeric characters
        // fn to_alphanumeric_string_lossy(name: &str) -> String {
        //     name.chars()
        //         .filter(|c| c.is_alphanumeric())
        //         .collect::<String>()
        // }

        // fn hash_file(location: &AssetSource, hash: &mut DefaultHasher) {
        //     // Hash the last time the file was updated and the file source. If either of these change, we need to regenerate the unique name
        //     let updated = location.last_updated();
        //     updated.hash(hash);
        //     location.hash(hash);
        // }

        todo!()
    }

    // /// Covnert the asset source to a string
    // pub fn raw(&self) -> String {
    //     match self {
    //         Self::Local(path) => path.relative.display().to_string(),
    //         Self::Remote(url) => url.to_string(),
    //     }
    // }

    // /// Try to convert the asset source to a local asset source
    // pub fn local(&self) -> Option<&AssetSource> {
    //     match self {
    //         Self::Local(path) => Some(path),
    //         Self::Remote(_) => None,
    //     }
    // }

    // /// Try to convert the asset source to a path
    // pub fn as_path(&self) -> Option<&PathBuf> {
    //     match self {
    //         Self::Local(path) => Some(&path.canonicalized),
    //         Self::Remote(_) => None,
    //     }
    // }

    // /// Try to convert the asset source to a url
    // pub fn as_url(&self) -> Option<&Url> {
    //     match self {
    //         Self::Local(_) => None,
    //         Self::Remote(url) => Some(url),
    //     }
    // }

    // /// Returns the last segment of the file source used to generate a unique name
    // pub fn last_segment(&self) -> &str {
    //     match self {
    //         Self::Local(path) => path.canonicalized.file_name().unwrap().to_str().unwrap(),
    //         Self::Remote(url) => url.path_segments().unwrap().last().unwrap(),
    //     }
    // }

    /// Returns the extension of the file source
    pub fn extension(&self) -> Option<String> {
        //     match self {
        //         Self::Local(path) => path
        //             .canonicalized
        //             .extension()
        //             .map(|e| e.to_str().unwrap().to_string()),
        //         Self::Remote(url) => reqwest::blocking::get(url.as_str())
        //             .ok()
        //             .and_then(|request| {
        //                 request
        //                     .headers()
        //                     .get("content-type")
        //                     .and_then(|content_type| {
        //                         content_type
        //                             .to_str()
        //                             .ok()
        //                             .map(|ty| ext_of_mime(ty).to_string())
        //                     })
        //             }),
        //     }
        todo!()
    }

    // /// Attempts to get the mime type of the file source
    // pub fn mime_type(&self) -> Option<String> {
    //     match self {
    //         Self::Local(path) => get_mime_from_path(&path.canonicalized)
    //             .ok()
    //             .map(|mime| mime.to_string()),
    //         Self::Remote(url) => reqwest::blocking::get(url.as_str())
    //             .ok()
    //             .and_then(|request| {
    //                 request
    //                     .headers()
    //                     .get("content-type")
    //                     .and_then(|content_type| Some(content_type.to_str().ok()?.to_string()))
    //             }),
    //     }
    // }

    // /// Find when the asset was last updated
    // pub fn last_updated(&self) -> Option<String> {
    //     match self {
    //         Self::Local(path) => path.canonicalized.metadata().ok().and_then(|metadata| {
    //             metadata
    //                 .modified()
    //                 .ok()
    //                 .map(|modified| format!("{:?}", modified))
    //                 .or_else(|| {
    //                     metadata
    //                         .created()
    //                         .ok()
    //                         .map(|created| format!("{:?}", created))
    //                 })
    //         }),
    //         Self::Remote(url) => reqwest::blocking::get(url.as_str())
    //             .ok()
    //             .and_then(|request| {
    //                 request
    //                     .headers()
    //                     .get("last-modified")
    //                     .and_then(|last_modified| {
    //                         last_modified
    //                             .to_str()
    //                             .ok()
    //                             .map(|last_modified| last_modified.to_string())
    //                     })
    //             }),
    //     }
    // }

    /// Reads the file to a string
    pub fn read_to_string(&self) -> anyhow::Result<String> {
        //     match &self {
        //         AssetSource::Local(path) => Ok(std::fs::read_to_string(&path.canonicalized)
        //             .with_context(|| {
        //                 format!(
        //                     "Failed to read file from location: {}",
        //                     path.canonicalized.display()
        //                 )
        //             })?),
        //         AssetSource::Remote(url) => {
        //             let response = reqwest::blocking::get(url.as_str())
        //                 .with_context(|| format!("Failed to asset from url: {}", url.as_str()))?;
        //             Ok(response.text().with_context(|| {
        //                 format!("Failed to read text for asset from url: {}", url.as_str())
        //             })?)
        //         }
        //     }
        todo!()
    }

    /// Reads the file to bytes
    pub fn read_to_bytes(&self) -> anyhow::Result<Vec<u8>> {
        //     match &self {
        //         AssetSource::Local(path) => {
        //             Ok(std::fs::read(&path.canonicalized).with_context(|| {
        //                 format!(
        //                     "Failed to read file from location: {}",
        //                     path.canonicalized.display()
        //                 )
        //             })?)
        //         }
        //         AssetSource::Remote(url) => {
        //             let response = reqwest::blocking::get(url.as_str())
        //                 .with_context(|| format!("Failed to asset from url: {}", url.as_str()))?;
        //             Ok(response.bytes().map(|b| b.to_vec()).with_context(|| {
        //                 format!("Failed to read text for asset from url: {}", url.as_str())
        //             })?)
        //         }
        //     }
        todo!()
    }

    /// The location where the asset will be served from post-bundle
    /// This is not the "resolved" location at runtime
    pub fn served_location(&self) -> Result<String, ()> {
        todo!()
    }

    // /// Returns the unique name of the file that the asset will be served from
    // pub fn unique_name(&self) -> &str {
    //     &self.unique_name
    // }

    // /// Returns the source of the file that the asset will be collected from
    // pub fn source(&self) -> &AssetSource {
    //     &self.source
    // }

    /// Returns the location of the file asset
    pub fn location(&self) -> &ResourceAsset {
        todo!()
        // &self.location
    }

    /// Returns the options for the file asset
    pub fn options(&self) -> &FileOptions {
        todo!()
        // &self.options
    }

    /// Returns the options for the file asset mutably
    pub fn with_options_mut(&mut self, f: impl FnOnce(&mut FileOptions)) {
        todo!()
        // f(&mut self.options);
        // self.regenerate_unique_name();
    }

    /// Regenerates the unique name of the file asset
    fn regenerate_unique_name(&mut self) {
        // // Generate an unique name for the file based on the options, source, and the current version of manganis
        // let uuid = self.hash();
        // let extension = self.options.extension();
        // let file_name = normalized_file_name(&self.location.source, extension);
        // let extension = extension.map(|e| format!(".{e}")).unwrap_or_default();
        // self.location.unique_name = format!("{file_name}{uuid:x}{extension}");
        // assert!(self.location.unique_name.len() <= MAX_PATH_LENGTH);
    }

    // /// Hash the file asset source and options
    // fn hash(&self) -> u64 {
    //     let mut hash = std::collections::hash_map::DefaultHasher::new();
    //     hash_file(&self.location.source, &mut hash);
    //     self.options.hash(&mut hash);
    //     hash_version(&mut hash);
    //     hash.finish()
    // }
}

/// Get the mime type from a URI using its extension
fn ext_of_mime(mime: &str) -> &str {
    let mime = mime.split(';').next().unwrap_or_default();
    match mime.trim() {
        "application/octet-stream" => "bin",
        "text/css" => "css",
        "text/csv" => "csv",
        "text/html" => "html",
        "image/vnd.microsoft.icon" => "ico",
        "text/javascript" => "js",
        "application/json" => "json",
        "application/ld+json" => "jsonld",
        "application/rtf" => "rtf",
        "image/svg+xml" => "svg",
        "video/mp4" => "mp4",
        "text/plain" => "txt",
        "application/xml" => "xml",
        "application/zip" => "zip",
        "image/png" => "png",
        "image/jpeg" => "jpg",
        "image/gif" => "gif",
        "image/webp" => "webp",
        "image/avif" => "avif",
        "font/ttf" => "ttf",
        "font/woff" => "woff",
        "font/woff2" => "woff2",
        other => other.split('/').last().unwrap_or_default(),
    }
}

/// Get the mime type from a path-like string
fn get_mime_from_path(trimmed: &Path) -> std::io::Result<&'static str> {
    if trimmed.extension().is_some_and(|ext| ext == "svg") {
        return Ok("image/svg+xml");
    }

    let res = match infer::get_from_path(trimmed)?.map(|f| f.mime_type()) {
        Some(f) => {
            if f == "text/plain" {
                get_mime_by_ext(trimmed)
            } else {
                f
            }
        }
        None => get_mime_by_ext(trimmed),
    };

    Ok(res)
}

/// Get the mime type from a URI using its extension
fn get_mime_by_ext(trimmed: &Path) -> &'static str {
    get_mime_from_ext(trimmed.extension().and_then(|e| e.to_str()))
}

/// Get the mime type from a URI using its extension
pub fn get_mime_from_ext(extension: Option<&str>) -> &'static str {
    match extension {
        Some("bin") => "application/octet-stream",
        Some("css") => "text/css",
        Some("csv") => "text/csv",
        Some("html") => "text/html",
        Some("ico") => "image/vnd.microsoft.icon",
        Some("js") => "text/javascript",
        Some("json") => "application/json",
        Some("jsonld") => "application/ld+json",
        Some("mjs") => "text/javascript",
        Some("rtf") => "application/rtf",
        Some("svg") => "image/svg+xml",
        Some("mp4") => "video/mp4",
        Some("png") => "image/png",
        Some("jpg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("webp") => "image/webp",
        Some("avif") => "image/avif",
        Some("txt") => "text/plain",
        // Assume HTML when a TLD is found for eg. `dioxus:://dioxuslabs.app` | `dioxus://hello.com`
        Some(_) => "text/html",
        // https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types/Common_types
        // using octet stream according to this:
        None => "application/octet-stream",
    }
}

fn hash_version(hash: &mut DefaultHasher) {
    // Hash the current version of manganis. If this changes, we need to regenerate the unique name
    crate::built::PKG_VERSION.hash(hash);
    crate::built::GIT_COMMIT_HASH.hash(hash);
}
