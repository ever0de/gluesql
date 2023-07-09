use {
    super::declare_trait,
    crate::{
        data::CustomFunction as StructCustomFunction,
        result::{Error, Result},
    },
    async_trait::async_trait,
};

macro_rules! custom_function {
    (#[$attr: meta]) => {
        declare_trait!(
            "By implementing `CustomFunction` trait, you can run `CREATE FUNCTION`, `DROP FUNCTION`, `SELECT` query"
            #[$attr]
            trait CustomFunction {
                async fn fetch_function(&self, _: &str) -> Result<Option<&StructCustomFunction>> {
                    Err(Error::StorageMsg(
                        "[Storage] CustomFunction is not supported".to_owned(),
                    ))
                }

                async fn fetch_all_functions(&self) -> Result<Vec<&StructCustomFunction>> {
                    Err(Error::StorageMsg(
                        "[Storage] CustomFunction is not supported".to_owned(),
                    ))
                }
            }
        );
    };
}

#[cfg(not(feature = "send"))]
custom_function!(#[async_trait(?Send)]);
#[cfg(feature = "send")]
custom_function!(#[async_trait]);

macro_rules! custom_function_mut {
    (#[$attr: meta]) => {
        declare_trait!(
            "By implementing `CustomFunctionMut` trait, you can run `CREATE FUNCTION`, `DROP FUNCTION`, `SELECT` query"
            #[$attr]
            trait CustomFunctionMut {
                async fn insert_function(&mut self, _: StructCustomFunction) -> Result<()> {
                    Err(Error::StorageMsg(
                        "[Storage] CustomFunction is not supported".to_owned(),
                    ))
                }

                async fn delete_function(&mut self, _: &str) -> Result<()> {
                    Err(Error::StorageMsg(
                        "[Storage] CustomFunction is not supported".to_owned(),
                    ))
                }
            }
        );
    };
}

#[cfg(not(feature = "send"))]
custom_function_mut!(#[async_trait(?Send)]);
#[cfg(feature = "send")]
custom_function_mut!(#[async_trait]);
