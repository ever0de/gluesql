use proc_macro2::{Literal, Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens};
use syn::{punctuated::Punctuated, token::Add, ItemTrait, TypeParamBound};

use crate::args::Args;

type SuperTraits = Punctuated<TypeParamBound, Add>;

pub fn generate(source_trait: &ItemTrait, args: &Args) {
    let mut features = quote! {};
    let mut trait_items = quote! {};

    for pairs in args.for_all_pairs() {
        for (feature_expr, trait_expr) in pairs {
            println!("{:?}\n{:?}", feature_expr, trait_expr);

            match &feature_expr {
                syn::Expr::Path(_) => {
                    //     features.extend();
                }
                _ => todo!(),
            }

            //     trait_items.extend(trait_expr.to_token_stream())
        }
    }
}
