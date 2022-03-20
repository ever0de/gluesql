use proc::feature_trait_bound;

use super::*;

#[feature_trait_bound([("alter-table", AlterTable), ("index", Index)])]
pub trait Test<T> {}
