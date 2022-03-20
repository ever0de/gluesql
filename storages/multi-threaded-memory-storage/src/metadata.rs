use {
    super::MultiThreadedMemoryStorage,
    async_trait::async_trait,
    gluesql_core::{result::Result, store::Metadata},
    std::sync::Arc,
};

#[async_trait(?Send)]
impl Metadata for MultiThreadedMemoryStorage {
    async fn schema_names(&self) -> Result<Vec<String>> {
        let items = Arc::clone(&self.items);
        let items = items.read().await;

        let mut names: Vec<_> = items.keys().map(Clone::clone).collect();
        names.sort();

        Ok(names)
    }
}