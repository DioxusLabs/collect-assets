use manganis_common::{AssetType, ManganisSupportError, ResourceAsset};
use quote::{quote, ToTokens};
use syn::{parenthesized, parse::Parse};

use crate::{generate_link_section, resource::ResourceAssetParser};

pub struct FileAssetParser {
    pub asset: ResourceAssetParser,
}

impl Parse for FileAssetParser {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let inside;
        parenthesized!(inside in input);

        let asset = inside.parse::<ResourceAssetParser>()?;

        Ok(FileAssetParser { asset })
    }
}

impl ToTokens for FileAssetParser {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.asset.to_tokens(tokens)
    }
}
