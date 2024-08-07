use manganis_common::{AssetSource, AssetType, FolderAsset, ManganisSupportError};
use quote::{quote, ToTokens};
use syn::{parenthesized, parse::Parse};

use crate::generate_link_section;

pub struct FolderAssetParser {
    file_name: Result<String, ManganisSupportError>,
    asset: AssetType,
}

impl Parse for FolderAssetParser {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let inside;
        parenthesized!(inside in input);
        let path = inside.parse::<syn::LitStr>()?;

        let path_as_str = path.value();
        let path = match AssetSource::parse_folder(&path_as_str) {
            Ok(path) => path,
            Err(e) => return Err(syn::Error::new(proc_macro2::Span::call_site(), e)),
        };
        let this_file = FolderAsset::new(path);
        let asset = manganis_common::AssetType::Folder(this_file.clone());

        let file_name = this_file.served_location();

        Ok(FolderAssetParser { file_name, asset })
    }
}

impl ToTokens for FolderAssetParser {
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