use proc::feature_trait_bound;

#[feature_trait_bound([("alter-table", AlterTable), ("index", Index)])]
pub trait Test {}
