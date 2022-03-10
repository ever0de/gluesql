use itertools::Itertools;
use syn::{
    parse::{Parse, ParseStream, Result},
    punctuated::Punctuated,
    Expr, ExprArray, ExprTuple, Token,
};

#[derive(Clone, Debug)]
pub struct Args {
    /// [(feature, TraitName), ...]
    pub inner: Vec<ExprTuple>,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self> {
        let vars = Punctuated::<ExprArray, Token![,]>::parse_terminated(input)?;

        let parse_tuple = |expr: Expr| match expr {
            Expr::Tuple(expr) => Ok(expr),
            tokens => Err(syn::Error::new_spanned(tokens, "Not supported type")),
        };

        let inner = vars
            .into_iter()
            .map(|expr| expr.elems.into_iter().map(parse_tuple).collect::<Vec<_>>())
            .flatten()
            .collect::<Result<Vec<_>>>()?;

        return Ok(Args { inner });
    }
}

impl Args {
    pub fn for_all_pairs(&self) -> Vec<Vec<(Expr, Expr)>> {
        let tuple_list = self.clone().inner;

        let to_token_stream_list = |tuple: ExprTuple| {
            let list = tuple.elems.into_iter().collect::<Vec<_>>();

            let feature_token = list[0].clone();
            let trait_token = list[1].clone();

            (feature_token, trait_token)
        };

        tuple_list
            .into_iter()
            .map(to_token_stream_list)
            .powerset()
            .filter(|x| !x.is_empty())
            .collect()
    }

    pub fn all_features(&self) -> Vec<Expr> {
        let tuple_list = self.clone().inner;

        let to_token_stream_list = |tuple: ExprTuple| {
            let list = tuple.elems.into_iter().collect::<Vec<_>>();
            let feature_token = list[0].clone();
            feature_token
        };

        tuple_list.into_iter().map(to_token_stream_list).collect()
    }
}
