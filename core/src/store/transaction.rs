use {
    super::declare_trait,
    crate::result::{Error, Result},
    async_trait::async_trait,
};

macro_rules! transaction {
    (#[$attr: meta]) => {
        declare_trait!(
            "By implementing `Transaction` trait, you can run `BEGIN`, `COMMIT`, `ROLLBACK` query"
            #[$attr]
            trait Transaction {
                async fn begin(&mut self, autocommit: bool) -> Result<bool> {
                    if autocommit {
                        return Ok(false);
                    }

                    Err(Error::StorageMsg(
                        "[Storage] Transaction::begin is not supported".to_owned(),
                    ))
                }

                async fn rollback(&mut self) -> Result<()> {
                    Ok(())
                }

                async fn commit(&mut self) -> Result<()> {
                    Ok(())
                }
            }
        );
    };
}

#[cfg(not(feature = "send"))]
transaction!(#[async_trait(?Send)]);
#[cfg(feature = "send")]
transaction!(#[async_trait]);
