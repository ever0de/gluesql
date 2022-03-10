use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{punctuated::Punctuated, token::Add, ItemTrait, TypeParamBound};

use crate::args::Args;

pub fn generate(source_trait: &ItemTrait, args: &Args) -> TokenStream2 {
    let mut source_list = quote! {};

    for pairs in args.for_all_pairs() {
        if pairs.is_empty() {
            source_list.extend(quote! {
                #source_trait,
            });
            continue;
        }

        let mut feature_list = quote! {};
        let mut trait_name_list = quote! {};
        pairs.iter().for_each(|(feature_expr, trait_expr)| {
            feature_list.extend(quote! {
                feature = #feature_expr,
            });
            trait_name_list.extend(quote! {#trait_expr});
        });

        source_list.extend(quote! {
            // #[cfg(all(#feature_list))]
            // #source_trait
        });
    }

    source_list
}
