/// Create an folder asset from the local path
///
/// > **Note**: This will do nothing outside of the `asset!` macro
///
/// The folder builder collects an arbitrary local folder. Relative paths are resolved relative to the package root
/// ```rust
/// const _: &str = manganis::asset!("/assets");
/// ```
#[allow(unused)]
pub const fn folder(path: &'static str) -> &'static str {
    path
}
