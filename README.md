# Manganis

> Like `include_bytes!`, but way better, and works with bundlers

The Manganis allows you to submit assets to a build tool that supports collecting assets. It makes it easy to self-host assets that are distributed throughout your library. Manganis also handles optimizing, converting, and fetching assets.

****
If you defined this in a component library:
```rust
const AVIF_ASSET: &str = manganis::asset!("/assets/rustacean-flat-gesture.png");
```

AVIF_ASSET will be set to a new file name that will be served by some CLI. That file can be collected by any package that depends on the component library.

```rust
// You can include tailwind classes that will be collected into the final binary
const TAILWIND_CLASSES: &str = manganis::classes!("flex flex-col p-5");

// You can also collect arbitrary files. Relative paths are resolved relative to the package root
const _: &str = manganis::asset!("test-package-dependency/src/asset.txt");

// You can use URLs to copy read the asset at build time
const _: &str = manganis::asset!("https://rustacean.net/assets/rustacean-flat-happy.png");

// You can collect images which will be automatically optimized
pub const PNG_ASSET: manganis::ImageAsset =
    manganis::asset!(image("rustacean-flat-gesture.png"));

// Resize the image at compile time to make the assets smaller
pub const RESIZED_PNG_ASSET: manganis::ImageAsset =
    manganis::asset!(image("rustacean-flat-gesture.png").size(52, 52));

// Or convert the image at compile time to a web friendly format
pub const AVIF_ASSET: manganis::ImageAsset = manganis::asset!(image("rustacean-flat-gesture.png")
    .format(ImageType::Avif));

// You can even include a low quality preview of the image embedded into the url
pub const AVIF_ASSET_LOW: manganis::ImageAsset = manganis::asset!(image("rustacean-flat-gesture.png")
	.format(ImageType::Avif)
	.low_quality_preview());

// You can also collect google fonts
pub const ROBOTO_FONT: &str = manganis::asset!(font()
    .families(["Roboto"]));

// Specify weights for fonts to collect
pub const COMFORTAA_FONT: &str = manganis::asset!(font()
    .families(["Comfortaa"])
    .weights([400]));

// Or specific text to include only the characters used in that text
pub const ROBOTO_FONT_LIGHT_FONT: &str = manganis::asset!(font()
    .families(["Roboto"])
    .weights([200])
    .text("hello world"));
```

## Adding Support to Your CLI

To add support for your CLI, you need to integrate with the [manganis_cli_support](https://github.com/DioxusLabs/manganis/tree/main/cli-support) crate. This crate provides utilities to collect assets that integrate with the Manganis macro. It makes it easy to integrate an asset collection and optimization system into a build tool.
