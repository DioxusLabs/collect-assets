use crate::file::FileAssetParser;
use crate::folder::FolderAssetParser;
use crate::font::FontAssetParser;
use crate::image::ImageAssetParser;
use crate::js::JsAssetParser;
use crate::json::JsonAssetParser;
use crate::{css::CssAssetParser, resource::ResourceAssetParser};

// use manganis_common::cache::macro_log_file;
use core::panic;
use manganis_common::{MetadataAsset, ResourceAsset, TailwindAsset};
use proc_macro::TokenStream;
use proc_macro2::Ident;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, quote_spanned, ToTokens};
use serde::Serialize;
use std::sync::atomic::Ordering;
use std::{fs::File, sync::atomic::AtomicBool};
use syn::{parenthesized, parse::Parse, parse_macro_input, LitStr};

#[derive(Copy, Clone, Default, PartialEq)]
enum ReturnType {
    #[default]
    AssetSpecific,
    StaticStr,
}

pub struct AssetParser {
    return_type: ReturnType,
    asset_type: syn::Result<RefinedAsset>,
    source: TokenStream2,
}

impl Parse for AssetParser {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // we can take
        //
        // "myfile.png".image().option1().option2()
        //
        // // gives you the FolderAsset type / etc
        // "myfile".folder()
        //
        // // gives you just the Asset type - it's generic and basically unrefined
        // "myfile"
        //
        // But we need to decide the hint first before parsing the options

        // Parse as an expr
        let expr = input.parse::<syn::Expr>()?;

        // And then match - is it a literal or a method call?
        // the method call will be a recursive chain
        match expr {
            syn::Expr::Lit(lit) => {}
            syn::Expr::MethodCall(call) => {}
            _ => todo!(),
        }

        // // The simple case where we just pass a string
        // if input.peek(syn::LitStr) && !input.peek2(syn::Token![.]) {
        //     //
        // }

        // // Otherwise, parse as a method call (which is recursive...)
        // let call = input.parse::<syn::ExprMethodCall>()?;

        // BY default, we're dealing with an asset
        // let mut asset_type = RefinedAsset::Asset;
        // let resource = input.parse::<ResourceAssetParser>()?;

        // If there's more tokens, we're doing a refinement and outputting a different type
        // Just parse into a list of method calls
        // if !input.is_empty() {
        //     let mut methods = Vec::new();
        //     while !input.is_empty() {
        //         methods.push(input.parse::<syn::ExprMethodCall>())
        //         // methods.push(input.parse::<RefinedAsset>()?);
        //     }
        //     // asset_type = RefinedAsset::Refined(methods);
        // }

        todo!()
    }
}

impl ToTokens for AssetParser {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        // let asset = match &self.asset_type {
        //     Ok(AnyAssetParserType::File(file)) => file.into_token_stream(),
        //     Ok(AnyAssetParserType::Folder(folder)) => folder.into_token_stream(),
        //     Ok(AnyAssetParserType::Image(image)) => {
        //         let tokens = image.into_token_stream();
        //         if self.return_type == ReturnType::StaticStr {
        //             quote! {
        //                 #tokens.path()
        //             }
        //         } else {
        //             tokens
        //         }
        //     }
        //     Ok(AnyAssetParserType::Font(font)) => font.into_token_stream(),
        //     Ok(AnyAssetParserType::Css(css)) => css.into_token_stream(),
        //     Ok(AnyAssetParserType::Js(js)) => js.into_token_stream(),
        //     Ok(AnyAssetParserType::Json(js)) => js.into_token_stream(),
        //     Err(e) => e.to_compile_error(),
        // };

        // partial expansion? imports? autocomplete?
        let source = &self.source;
        let source = quote! {
            const _: &dyn manganis::ForMgMacro = {
                use manganis::*;
                &#source
            };
        };

        tokens.extend(quote! {
            {
                #source
                #asset
            }
        })
    }
}

enum RefinedAsset {
    Asset,
    File,
    Folder,
    Image,
    Font,
    Css,
    Js,
    Json,
}

// impl Parse for AnyAssetParserType {
//     fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
//         let ident = input.parse::<syn::Ident>()?;
//         let as_string = ident.to_string();

//         Ok(match &*as_string {
//             "file" => Self::File(input.parse::<FileAssetParser>()?),
//             "folder" => Self::Folder(input.parse::<FolderAssetParser>()?),
//             "image" => Self::Image(input.parse::<ImageAssetParser>()?),
//             "font" => Self::Font(input.parse::<FontAssetParser>()?),
//             "css" => Self::Css(input.parse::<CssAssetParser>()?),
//             "js" => Self::Js(input.parse::<JsAssetParser>()?),
//             "json" => Self::Json(input.parse::<JsonAssetParser>()?),
//             _ => {
//                 return Err(syn::Error::new(
//                     proc_macro2::Span::call_site(),
//                     format!(
//                         "Unknown asset type: {as_string}. Supported types are file, image, font, and css"
//                     ),
//                 ))
//             }
//         })
//     }
// }

// Default to the file asset type
// let mut asset_type = AnyAssetParserType::File;

// // If we see an ident, we know we're dealing with a resource
// let resource = if input.peek(syn::Ident) {
//     let ident = input.parse::<syn::Ident>()?;
//     asset_type = match ident.to_string().as_str() {
//         "file" => AnyAssetParserType::File,
//         "folder" => AnyAssetParserType::Folder,
//         "image" => AnyAssetParserType::Image,
//         "font" => AnyAssetParserType::Font,
//         "css" => AnyAssetParserType::Css,
//         "js" => AnyAssetParserType::Js,
//         "json" => AnyAssetParserType::Json,
//         _ => panic!("Unknown asset type: {}", ident),
//     };
//     let parantheted;
//     parenthesized!(parantheted in input);
//     parantheted.parse::<ResourceAssetParser>()?
// } else {
//     input.parse::<ResourceAssetParser>()?
// };

// Read the file from the FS if we need to and refine it if necessary?

// // First try to parse `"myfile".option1().option2()`. We parse that like asset_type("myfile.png").option1().option2()
// if input.peek(syn::LitStr) {
//     // let asset = input.parse::<ResourceAssetParser>()?;
//     let builder_tokens = { input.fork().parse::<TokenStream2>()? };
//     let asset = input.parse::<ResourceAssetParser>()?;
//     let asset = FileAssetParser { asset };

//     return Ok(AssetParser {
//         return_type: ReturnType::StaticStr,
//         asset_type: Ok(AnyAssetParserType::File(asset)),
//         source: builder_tokens,
//         // asset_type: Ok(AnyAssetParserType::File(asset)),
//         // source: input.parse()?,
//     });

//     // let path_str = input.parse::<syn::LitStr>()?;
//     // // Try to parse an extension
//     // let asset = ResourceAsset::parse_any(&path_str.value())
//     //     .map_err(|e| syn::Error::new(proc_macro2::Span::call_site(), e))?;
//     // let input: proc_macro2::TokenStream = input.parse()?;
//     // let parse_asset = || -> syn::Result<Self> {
//     //     if let Some(extension) = asset.extension() {
//     //         if extension.parse::<manganis_common::ImageType>().is_ok() {
//     //             return syn::parse2(
//     //                 quote_spanned! { path_str.span() => image(#path_str) #input },
//     //             );
//     //         } else if extension.parse::<manganis_common::VideoType>().is_ok() {
//     //             return syn::parse2(
//     //                 quote_spanned! { path_str.span() => video(#path_str) #input },
//     //             );
//     //         }
//     //     }
//     //     // if let ResourceAsset::Local(path) = &asset {
//     //     //     if path.canonicalized.is_dir() {
//     //     //         return syn::parse2(
//     //     //             quote_spanned! { path_str.span() => folder(#path_str) #input },
//     //     //         );
//     //     //     }
//     //     // }
//     //     syn::parse2(quote_spanned! { path_str.span() => file(#path_str) #input })
//     // };

//     // let mut asset = parse_asset()?;
//     // // We always return a static string if the asset was not parsed with an explicit type
//     // asset.return_type = ReturnType::StaticStr;
//     // return Ok(asset);
// }

// let builder_tokens = { input.fork().parse::<TokenStream2>()? };
// let asset = input.parse::<AnyAssetParserType>();

// Ok(AssetParser {
//     return_type: ReturnType::AssetSpecific,
//     asset_type: asset,
//     source: builder_tokens,
// })
