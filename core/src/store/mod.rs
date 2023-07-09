mod alter_table;
mod data_row;
mod function;
mod index;
mod metadata;
mod transaction;

pub trait GStore: Store + Index + Metadata + CustomFunction {}
impl<S: Store + Index + Metadata + CustomFunction> GStore for S {}

pub trait GStoreMut:
    StoreMut + IndexMut + AlterTable + Transaction + CustomFunction + CustomFunctionMut
{
}
impl<S: StoreMut + IndexMut + AlterTable + Transaction + CustomFunction + CustomFunctionMut>
    GStoreMut for S
{
}

pub use {
    alter_table::{AlterTable, AlterTableError},
    data_row::DataRow,
    function::{CustomFunction, CustomFunctionMut},
    index::{Index, IndexError, IndexMut},
    metadata::{MetaIter, Metadata},
    transaction::Transaction,
};

use {
    crate::{
        data::{Key, Schema},
        result::Result,
    },
    async_trait::async_trait,
};

pub type RowIter = Box<dyn Iterator<Item = Result<(Key, DataRow)>>>;

macro_rules! declare_trait {
    (#[$attr: meta] trait $name: ident $methods:tt) => {
        #[$attr]
        pub trait $name $methods
    };
    (
        $doc_comment: literal
        #[$attr: meta] trait $name: ident $methods:tt
    ) => {
        #[doc = $doc_comment]
        #[$attr]
        pub trait $name $methods
    }
}

pub(super) use declare_trait;

macro_rules! store {
    (#[$attr: meta]) => {
        declare_trait!(
            "By implementing `Store` trait, you can run `SELECT` query"
            #[$attr]
            trait Store {
                async fn fetch_schema(&self, table_name: &str) -> Result<Option<Schema>>;

                async fn fetch_all_schemas(&self) -> Result<Vec<Schema>>;

                async fn fetch_data(&self, table_name: &str, key: &Key) -> Result<Option<DataRow>>;

                async fn scan_data(&self, table_name: &str) -> Result<RowIter>;
            }
        );
    };
}

#[cfg(not(feature = "send"))]
store!(#[async_trait(?Send)]);
#[cfg(feature = "send")]
store!(#[async_trait]);

macro_rules! store_mut {
    ( #[$attr : meta] ) => {
        declare_trait ! (
            "By implementing `StoreMut` trait, you can run `INSERT`, `CREATE TABLE`, `DELETE`, `UPDATE` and `DROP TABLE` queries."
            #[$attr]
            trait StoreMut {
                async fn insert_schema(&mut self, schema: &Schema) -> Result<()>;

                async fn delete_schema(&mut self, table_name: &str) -> Result<()>;

                async fn append_data(&mut self, table_name: &str, rows: Vec<DataRow>) -> Result<()>;

                async fn insert_data(&mut self, table_name: &str, rows: Vec<(Key, DataRow)>) -> Result<()>;

                async fn delete_data(&mut self, table_name: &str, keys: Vec<Key>) -> Result<()>;
            }
        );
    };
}

#[cfg(not(feature = "send"))]
store_mut!(#[async_trait(?Send)]);
#[cfg(feature = "send")]
store_mut!(#[async_trait]);
