use manganis_common::{AssetSource, AssetType, CssOptions, FileAsset, ManganisSupportError};
use quote::{quote, ToTokens};
use syn::{bracketed, parenthesized, parse::Parse};

use crate::generate_link_section;

#[derive(Default)]
struct FontFamilies {
    families: Vec<String>,
}

impl Parse for FontFamilies {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let inside;
        bracketed!(inside in input);
        let array =
            syn::punctuated::Punctuated::<syn::LitStr, syn::Token![,]>::parse_separated_nonempty(
                &inside,
            )?;
        Ok(FontFamilies {
            families: array.into_iter().map(|f| f.value()).collect(),
        })
    }
}

#[derive(Default)]
struct FontWeights {
    weights: Vec<u32>,
}

impl Parse for FontWeights {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let inside;
        bracketed!(inside in input);
        let array =
            syn::punctuated::Punctuated::<syn::LitInt, syn::Token![,]>::parse_separated_nonempty(
                &inside,
            )?;
        Ok(FontWeights {
            weights: array
                .into_iter()
                .map(|f| f.base10_parse().unwrap())
                .collect(),
        })
    }
}

struct ParseFontOptions {
    families: FontFamilies,
    weights: FontWeights,
    text: Option<String>,
    display: Option<String>,
}

impl ParseFontOptions {
    fn url(&self) -> String {
        let mut segments = Vec::new();

        let families: Vec<_> = self
            .families
            .families
            .iter()
            .map(|f| f.replace(' ', "+"))
            .collect();
        if !families.is_empty() {
            segments.push(format!("family={}", families.join("&")));
        }

        let weights: Vec<_> = self.weights.weights.iter().map(|w| w.to_string()).collect();
        if !weights.is_empty() {
            segments.push(format!("weight={}", weights.join(",")));
        }

        if let Some(text) = &self.text {
            segments.push(format!("text={}", text.replace(' ', "+")));
        }

        if let Some(display) = &self.display {
            segments.push(format!("display={}", display.replace(' ', "+")));
        }

        let query = if segments.is_empty() {
            String::new()
        } else {
            format!("?{}", segments.join("&"))
        };

        format!("https://fonts.googleapis.com/css2{}", query)
    }
}

impl Parse for ParseFontOptions {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut families = None;
        let mut weights = None;
        let mut text = None;
        let mut display = None;
        loop {
            if input.is_empty() {
                break;
            }
            let _ = input.parse::<syn::Token![.]>()?;
            let ident = input.parse::<syn::Ident>()?;
            let inside;
            parenthesized!(inside in input);
            match ident.to_string().to_lowercase().as_str() {
                "families" => {
                    families = Some(inside.parse::<FontFamilies>()?);
                }
                "weights" => {
                    weights = Some(inside.parse::<FontWeights>()?);
                }
                "text" => {
                    text = Some(inside.parse::<syn::LitStr>()?.value());
                }
                "display" => {
                    display = Some(inside.parse::<syn::LitStr>()?.value());
                }
                _ => {
                    return Err(syn::Error::new(
                        proc_macro2::Span::call_site(),
                        format!("Unknown font option: {ident}. Supported options are families, weights, text, display"),
                    ))
                }
            }
        }

        Ok(ParseFontOptions {
            families: families.unwrap_or_default(),
            weights: weights.unwrap_or_default(),
            text,
            display,
        })
    }
}

pub struct FontAssetParser {
    file_name: Result<String, ManganisSupportError>,
    asset: AssetType,
}

impl Parse for FontAssetParser {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let inside;
        parenthesized!(inside in input);
        if !inside.is_empty() {
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "Font assets do not support paths. Please use file() if you want to import a local font file",
            ));
        }

        let options = input.parse::<ParseFontOptions>()?;

        let url = options.url();
        let url: AssetSource = match AssetSource::parse_file(&url) {
            Ok(url) => url,
            Err(e) => {
                return Err(syn::Error::new(
                    proc_macro2::Span::call_site(),
                    format!("Failed to parse url: {url:?}\n{e}"),
                ))
            }
        };
        let this_file = FileAsset::new(url.clone())
            .with_options(manganis_common::FileOptions::Css(CssOptions::default()));
        let asset = manganis_common::AssetType::File(this_file.clone());

        let file_name = this_file.served_location();

        Ok(FontAssetParser { file_name, asset })
    }
}

impl ToTokens for FontAssetParser {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let file_name = crate::quote_path(&self.file_name);

        let link_section = generate_link_section(self.asset.clone());

        tokens.extend(quote! {
            {
                #link_section
                #file_name
            }
        })
    }
}
