use {
    crate::{
        ast::Statement,
        executor::{execute, Payload},
        parse_sql::parse,
        plan::plan,
        result::Result,
        store::{GStore, GStoreMut},
        translate::translate,
    },
    futures::{
        executor::block_on,
        stream::{self, StreamExt},
        TryStreamExt,
    },
};

pub struct Glue<T: GStore + GStoreMut> {
    pub storage: T,
}

impl<T: GStore + GStoreMut> Glue<T> {
    pub fn new(storage: T) -> Self {
        Self { storage }
    }

    pub async fn plan(&self, sql: impl AsRef<str>) -> Result<Vec<Statement>> {
        let parsed = parse(sql)?;

        stream::iter(parsed)
            .map(|p| translate(&p))
            .then(|statement| async move { plan(&self.storage, statement?).await })
            .try_collect()
            .await
    }

    pub fn execute_stmt(&mut self, statement: Statement) -> Result<Payload> {
        block_on(self.execute_stmt_async(statement))
    }

    pub fn execute(&mut self, sql: impl AsRef<str>) -> Result<Vec<Payload>> {
        let statements = block_on(self.plan(sql))?;

        statements
            .into_iter()
            .map(|s| self.execute_stmt(s))
            .collect()
    }

    pub async fn execute_stmt_async(&mut self, statement: Statement) -> Result<Payload> {
        execute(&mut self.storage, statement).await
    }

    pub async fn execute_async(&mut self, sql: impl AsRef<str>) -> Result<Vec<Payload>> {
        let statements = self.plan(sql).await?;

        let mut payloads = Vec::<Payload>::new();
        for statement in statements {
            let payload = self.execute_stmt_async(statement).await?;
            payloads.push(payload);
        }

        Ok(payloads)
    }
}
