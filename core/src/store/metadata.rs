use {
    super::declare_trait,
    crate::{prelude::Value, result::Result},
    async_trait::async_trait,
    std::{collections::HashMap, iter::empty},
};

type ObjectName = String;
pub type MetaIter = Box<dyn Iterator<Item = Result<(ObjectName, HashMap<String, Value>)>>>;

macro_rules! metadata {
    (#[$attr: meta]) => {
        declare_trait!(
            "By implementing `Metadata` trait, you can run `SHOW` query"
            #[$attr]
            trait Metadata {
                async fn scan_table_meta(&self) -> Result<MetaIter> {
                    Ok(Box::new(empty()))
                }
            }
        );
    };
}

#[cfg(not(feature = "send"))]
metadata!(#[async_trait(?Send)]);
#[cfg(feature = "send")]
metadata!(#[async_trait]);
