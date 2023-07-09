use {
    super::Build,
    crate::{
        ast::Statement,
        executor::Payload,
        prelude::Glue,
        result::Result,
        store::{GStore, GStoreMut},
    },
    async_trait::async_trait,
};

#[async_trait(?Send)]
pub trait Execute<T>
where
    T: GStore + GStoreMut + Send + Sync,
    Self: Sized + Build,
{
    async fn execute(self, glue: &mut Glue<T>) -> Result<Payload> {
        let statement = self.build()?;

        glue.execute_stmt(&statement).await
    }
}

#[async_trait(?Send)]
impl<T: GStore + GStoreMut + Send + Sync, B: Build> Execute<T> for B {}

impl Build for Statement {
    fn build(self) -> Result<Statement> {
        Ok(self)
    }
}
