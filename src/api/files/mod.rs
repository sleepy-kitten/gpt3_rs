use serde::Serialize;

pub mod content;
pub mod delete;
pub mod list;
pub mod metadata;
pub mod upload;

#[derive(Debug, Clone, Serialize)]
pub struct File<T>
where
    T: Serialize,
{
    data: Vec<T>,
}
