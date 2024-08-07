use manganis_common::{AssetType, FolderAsset, ManganisSupportError, ResourceAsset};
use quote::{quote, ToTokens};
use syn::{parenthesized, parse::Parse};

use crate::{generate_link_section, resource::ResourceAssetParser};

pub struct FolderAssetParser {
    asset: ResourceAsset,
}

impl Parse for FolderAssetParser {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let inside;
        parenthesized!(inside in input);
        let path = inside.parse::<syn::LitStr>()?;

        let path_as_str = path.value();
        let asset = match ResourceAsset::parse_folder(&path_as_str) {
            Ok(path) => path,
            Err(e) => return Err(syn::Error::new(proc_macro2::Span::call_site(), e)),
        };

        Ok(FolderAssetParser { asset })
    }
}

impl ToTokens for FolderAssetParser {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        ResourceAssetParser::to_ref_tokens(&self.asset, tokens)
    }
}
