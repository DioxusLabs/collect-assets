const _: &str = manganis::classes!("flex flex-row p-4");
pub const CSS_ASSET: &str = manganis::mg!(file("./all_the_assets/style.css"));
pub const PNG_ASSET: &str = manganis::mg!(file("./all_the_assets/rustacean-flat-gesture.png"));
pub const RESIZED_PNG_ASSET: manganis::ImageAsset =
    manganis::mg!(image("./all_the_assets/rustacean-flat-gesture.png").size(52, 52));
pub const JPEG_ASSET: manganis::ImageAsset =
    manganis::mg!(image("./all_the_assets/rustacean-flat-gesture.png").format(ImageType::Jpg));
pub const RESIZED_JPEG_ASSET: manganis::ImageAsset =
    manganis::mg!(image("./all_the_assets/rustacean-flat-gesture.png")
        .format(ImageType::Jpg)
        .size(52, 52));
pub const AVIF_ASSET: manganis::ImageAsset =
    manganis::mg!(image("./all_the_assets/rustacean-flat-gesture.png").format(ImageType::Avif));
pub const RESIZED_AVIF_ASSET: manganis::ImageAsset =
    manganis::mg!(image("./all_the_assets/rustacean-flat-gesture.png")
        .format(ImageType::Avif)
        .size(52, 52));
pub const WEBP_ASSET: manganis::ImageAsset =
    manganis::mg!(image("./all_the_assets/rustacean-flat-gesture.png").format(ImageType::Webp));
pub const RESIZED_WEBP_ASSET: manganis::ImageAsset =
    manganis::mg!(image("./all_the_assets/rustacean-flat-gesture.png")
        .format(ImageType::Webp)
        .size(52, 52));
