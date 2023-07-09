use {
    super::declare_trait,
    crate::{
        ast::ColumnDef,
        result::{Error, Result},
    },
    async_trait::async_trait,
    serde::Serialize,
    std::fmt::Debug,
    thiserror::Error,
};

#[derive(Error, Serialize, Debug, PartialEq, Eq)]
pub enum AlterTableError {
    #[error("Table not found: {0}")]
    TableNotFound(String),

    #[error("Renaming column not found")]
    RenamingColumnNotFound,

    #[error("Default value is required: {0:#?}")]
    DefaultValueRequired(ColumnDef),

    #[error("Already existing column: {0}")]
    AlreadyExistingColumn(String),

    #[error("Dropping column not found: {0}")]
    DroppingColumnNotFound(String),

    #[error("Schemaless table does not support ALTER TABLE: {0}")]
    SchemalessTableFound(String),
}

macro_rules! alter_table {
    (#[$attr: meta]) => {
        declare_trait!(
            "By implementing `AlterTable` trait, you can run `ALTER TABLE` query"
            #[$attr]
            trait AlterTable {
                async fn rename_schema(&mut self, _: &str, _: &str) -> Result<()> {
                    Err(Error::StorageMsg(
                        "[Storage] AlterTable::rename_schema is not supported".to_owned(),
                    ))
                }

                async fn rename_column(&mut self, _: &str, _: &str, _: &str) -> Result<()> {
                    Err(Error::StorageMsg(
                        "[Storage] AlterTable::rename_column is not supported".to_owned(),
                    ))
                }

                async fn add_column(&mut self, _: &str, _: &ColumnDef) -> Result<()> {
                    Err(Error::StorageMsg(
                        "[Storage] AlterTable::add_column is not supported".to_owned(),
                    ))
                }

                async fn drop_column(&mut self, _: &str, _: &str, _: bool) -> Result<()> {
                    Err(Error::StorageMsg(
                        "[Storage] AlterTable::drop_column is not supported".to_owned(),
                    ))
                }
            }
        );
    };
}

#[cfg(not(feature = "send"))]
alter_table!(#[async_trait(?Send)]);
#[cfg(feature = "send")]
alter_table!(#[async_trait]);
