use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    parse_macro_input, punctuated::Punctuated, token::Colon2, AttributeArgs, Ident, ItemTrait,
    Path, PathArguments, PathSegment, TraitBound, TraitBoundModifier, TypeParamBound,
};

mod args;
mod parse;
mod supertraits;

use args::Args;

#[proc_macro_attribute]
pub fn feature_trait_bound(args: TokenStream, item: TokenStream) -> TokenStream {
    // item: pub trait GStore<T: Debug>: Store<T> {}
    let mut item = parse_macro_input!(item as ItemTrait);
    // [(feature, Trait), ...]
    let args = parse_macro_input!(args as Args);

    supertraits::generate(&item, &args);

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

    quote! {}.into()
}

// fn for_all_pairs(args: Args) -> ()
