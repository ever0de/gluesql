use std::ops::Add;

use async_trait::async_trait;

#[async_trait(?Send)]
pub trait AutoIncrement
where
    Self: Sized,
{
    type Key: Add;

    fn generate_id(&self) -> Self::Key {
        panic!()
    }
}
