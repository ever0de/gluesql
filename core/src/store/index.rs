use {
    super::{declare_trait, RowIter},
    crate::{
        ast::{IndexOperator, OrderByExpr},
        data::Value,
        result::{Error, Result},
    },
    async_trait::async_trait,
    serde::Serialize,
    std::fmt::Debug,
    thiserror::Error as ThisError,
};

#[derive(ThisError, Serialize, Debug, PartialEq, Eq)]
pub enum IndexError {
    #[error("table not found: {0}")]
    TableNotFound(String),

    #[error("index name already exists: {0}")]
    IndexNameAlreadyExists(String),

    #[error("index name does not exist: {0}")]
    IndexNameDoesNotExist(String),

    #[error("conflict - table not found: {0}")]
    ConflictTableNotFound(String),

    #[error("conflict - update failed - index value")]
    ConflictOnEmptyIndexValueUpdate,

    #[error("conflict - delete failed - index value")]
    ConflictOnEmptyIndexValueDelete,

    #[error("conflict - scan failed - index value")]
    ConflictOnEmptyIndexValueScan,

    #[error("conflict - index sync - delete index data")]
    ConflictOnIndexDataDeleteSync,
}

macro_rules! index {
    (#[$attr: meta]) => {
        declare_trait!(
            "By implementing `Index` trait, you can run `CREATE INDEX`, `DROP INDEX`, `SELECT` query"
            #[$attr]
            trait Index {
                async fn scan_indexed_data(
                    &self,
                    _table_name: &str,
                    _index_name: &str,
                    _asc: Option<bool>,
                    _cmp_value: Option<(&IndexOperator, Value)>,
                ) -> Result<RowIter> {
                    Err(Error::StorageMsg(
                        "[Storage] Index::scan_indexed_data is not supported".to_owned(),
                    ))
                }
            }
        );
    };
}

#[cfg(not(feature = "send"))]
index!(#[async_trait(?Send)]);
#[cfg(feature = "send")]
index!(#[async_trait]);

macro_rules! index_mut {
    (#[$attr: meta]) => {
        declare_trait!(
            "By implementing `IndexMut` trait, you can run `CREATE INDEX`, `DROP INDEX`, `SELECT` query"
            #[$attr]
            trait IndexMut {
                async fn create_index(
                    &mut self,
                    _table_name: &str,
                    _index_name: &str,
                    _column: &OrderByExpr,
                ) -> Result<()> {
                    Err(Error::StorageMsg(
                        "[Storage] Index::create_index is not supported".to_owned(),
                    ))
                }

                async fn drop_index(&mut self, _table_name: &str, _index_name: &str) -> Result<()> {
                    Err(Error::StorageMsg(
                        "[Storage] Index::drop_index is not supported".to_owned(),
                    ))
                }
            }
        );
    };
}

#[cfg(not(feature = "send"))]
index_mut!(#[async_trait(?Send)]);
#[cfg(feature = "send")]
index_mut!(#[async_trait]);
