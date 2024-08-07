use manganis_common::{AssetType, FolderAsset, ManganisSupportError, ResourceAsset};
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    token::Token,
};

use crate::generate_link_section;

pub struct ResourceAssetParser {
    pub asset: ResourceAsset,
}

impl ResourceAssetParser {
    pub fn new(asset: ResourceAsset) -> Self {
        Self { asset }
    }

    pub fn to_ref_tokens(asset: &ResourceAsset, tokens: &mut proc_macro2::TokenStream) {
        let link_section = generate_link_section(&asset);
        let input = asset.input.to_string();
        let local = asset.local.to_string();
        let bundled = asset.bundled.to_string();

        tokens.extend(quote! {
            {
                #link_section
                manganis::Asset {
                    input: #input,
                    local: #local,
                    bundled: #bundled,
                }
            }
        })
    }
}

/// Parse a litstr into a resource asset
impl Parse for ResourceAssetParser {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let s = input.parse::<syn::LitStr>()?;
        let asset =
            ResourceAsset::parse_any(&s.value()).map_err(|e| syn::Error::new(s.span(), e))?;

        Ok(Self { asset })
    }
}

impl ToTokens for ResourceAssetParser {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        Self::to_ref_tokens(&self.asset, tokens)
    }
}
