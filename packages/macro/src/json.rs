use manganis_common::{AssetType, FileOptions, ManganisSupportError, ResourceAsset};
use quote::{quote, ToTokens};
use syn::{parenthesized, parse::Parse};

use crate::{generate_link_section, resource::ResourceAssetParser};

pub struct JsonAssetParser {
    asset: ResourceAssetParser,
}

impl Parse for JsonAssetParser {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let inside;
        parenthesized!(inside in input);
        let mut asset = inside.parse::<ResourceAssetParser>()?;

        if !input.is_empty() {
            let options = input.parse::<ParseJsonOptions>()?;
            todo!()
            // asset.asset.set_options(FileOptions::Json(options.options));
        }

        Ok(JsonAssetParser { asset })
    }
}

impl ToTokens for JsonAssetParser {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.asset.to_tokens(tokens)
    }
}

struct ParseJsonOptions {
    options: Vec<ParseJsonOption>,
}

impl ParseJsonOptions {
    fn apply_to_options(self, file: &mut ResourceAsset) {
        for option in self.options {
            option.apply_to_options(file);
        }
    }
}

impl Parse for ParseJsonOptions {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut options = Vec::new();
        while !input.is_empty() {
            options.push(input.parse::<ParseJsonOption>()?);
        }
        Ok(ParseJsonOptions { options })
    }
}

enum ParseJsonOption {
    UrlEncoded(bool),
    Preload(bool),
}

impl ParseJsonOption {
    fn apply_to_options(self, file: &mut ResourceAsset) {
        match self {
            ParseJsonOption::Preload(preload) => file.with_options_mut(|options| {
                if let FileOptions::Json(options) = options {
                    options.set_preload(preload);
                }
            }),
            ParseJsonOption::UrlEncoded(url_encoded) => {
                file.set_url_encoded(url_encoded);
            }
        }
    }
}

impl Parse for ParseJsonOption {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _ = input.parse::<syn::Token![.]>()?;
        let ident = input.parse::<syn::Ident>()?;
        let _content;
        parenthesized!(_content in input);
        match ident.to_string().as_str() {
            "preload" => {
                crate::verify_preload_valid(&ident)?;
                Ok(ParseJsonOption::Preload(true))
            },
            "url_encoded" => Ok(ParseJsonOption::UrlEncoded(true)),
            _ => Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                format!(
                    "Unknown Json option: {}. Supported options are preload, url_encoded, and minify",
                    ident
                ),
            )),
        }
    }
}

// use syn::parse::Parser;
// ResourceAssetParser::parse_file.parse2(input.into())?;

// let path_as_str = path.value();
// let mut asset: ResourceAsset = match ResourceAsset::parse_file(&path_as_str) {
//     Ok(path) => path.with_options(manganis_common::FileOptions::Json(Default::default())),
//     Err(e) => {
//         return Err(syn::Error::new(
//             proc_macro2::Span::call_site(),
//             format!("{e}"),
//         ))
//     }
// };

// if let Some(parsed_options) = parsed_options {
//     parsed_options.apply_to_options(&mut asset);
// }

// let file_name = if asset.url_encoded() {
//     #[cfg(not(feature = "url-encoding"))]
//     return Err(syn::Error::new(
//         proc_macro2::Span::call_site(),
//         "URL encoding is not enabled. Enable the url-encoding feature to use this feature",
//     ));
//     #[cfg(feature = "url-encoding")]
//     Ok(crate::url_encoded_asset(&asset).map_err(|e| {
//         syn::Error::new(
//             proc_macro2::Span::call_site(),
//             format!("Failed to encode file: {}", e),
//         )
//     })?)
// } else {
//     asset.served_location()
// };
