use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens};
use syn::{
    parse_macro_input,
    punctuated::Punctuated,
    token::{Add, Colon2, Trait},
    AttributeArgs, Ident, ItemTrait, Path, PathArguments, PathSegment, TraitBound,
    TraitBoundModifier, TypeParamBound,
};

mod args;
mod parse;
mod supertraits;

use args::Args;

#[proc_macro_attribute]
pub fn feature_trait_bound(args: TokenStream, item: TokenStream) -> TokenStream {
    // item: pub trait GStore<T: Debug>: Store<T> {}
    let item = parse_macro_input!(item as ItemTrait);
    // [(feature, Trait), ...]
    let args = parse_macro_input!(args as Args);

    // supertraits::generate(&item, &args).into()

    let not_all_features = args.all_features().into_iter().map(|expr| {
        quote! { not(feature = #expr) }
    });
    let mut source = quote! {
        #[cfg(all(#(#not_all_features),*))]
        #item
    };

    for pairs in args.for_all_pairs() {
        let mut feature_list = vec![];
        let mut trait_name_list = vec![];
        pairs.iter().for_each(|(feature_expr, trait_expr)| {
            feature_list.push(quote! {
                feature = #feature_expr
            });

            let trait_name = quote! {#trait_expr}.to_string();
            trait_name_list.push(trait_name);
        });

        source.extend(quote! {
            #[cfg(all(#(#feature_list),*))]
            #item
        });
    }

    println!("{}", source.to_string());

    source.into()
}

// let mut segments = Punctuated::<PathSegment, Colon2>::new();
// segments.push(PathSegment {
//     ident: Ident::new("Metadata", Span::call_site()),
//     arguments: PathArguments::None,
// });
// item.supertraits.push(TypeParamBound::Trait(TraitBound {
//     paren_token: None,
//     modifier: TraitBoundModifier::None,
//     lifetimes: None,
//     path: Path {
//         leading_colon: None,
//         segments,
//     },
// }));
