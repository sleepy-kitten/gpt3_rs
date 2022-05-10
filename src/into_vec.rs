use std::ops::Deref;

use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Convenience type to allow using `&[&str]` instead of `Vec<String>` on the builder methods.
///
/// # Example
/// allows:
/// ```ignore
/// &["some", "thing"]
/// ```
/// instead of: 
/// ```ignore
/// vec!["some".into(), "thing".into()]
/// ```
pub struct IntoVec<T>(Vec<T>);

impl From<&[&str]> for IntoVec<String> {
    fn from(from: &[&str]) -> Self {
        IntoVec(from.iter().cloned().map(ToOwned::to_owned).collect())
    }
}
impl From<&&[&str]> for IntoVec<String> {
    fn from(from: &&[&str]) -> Self {
        IntoVec(from.iter().cloned().map(ToOwned::to_owned).collect())
    }
}
impl<const N: usize> From<&[&str; N]> for IntoVec<String> {
    fn from(from: &[&str; N]) -> Self {
        IntoVec(from.iter().cloned().map(ToOwned::to_owned).collect())
    }
}
impl From<Vec<&str>> for IntoVec<String> {
    fn from(from: Vec<&str>) -> Self {
        IntoVec(from.into_iter().map(ToOwned::to_owned).collect())
    }
}
impl From<&[&[&str]]> for IntoVec<Vec<String>> {
    fn from(from: &[&[&str]]) -> Self {
        IntoVec(
            from.iter()
                .cloned()
                .map(|rest| rest.iter().cloned().map(ToOwned::to_owned).collect())
                .collect(),
        )
    }
}
impl<const N1: usize, const N2: usize> From<&[&[&str; N1]; N2]> for IntoVec<Vec<String>> {
    fn from(from: &[&[&str; N1]; N2]) -> Self {
        IntoVec(
            from.iter()
                .cloned()
                .map(|rest| rest.iter().cloned().map(ToOwned::to_owned).collect())
                .collect(),
        )
    }
}
impl From<&&[&&[&str]]> for IntoVec<Vec<String>> {
    fn from(from: &&[&&[&str]]) -> Self {
        IntoVec(
            from.iter()
                .cloned()
                .map(|rest| rest.iter().cloned().map(ToOwned::to_owned).collect())
                .collect(),
        )
    }
}
impl From<Vec<Vec<&str>>> for IntoVec<Vec<String>> {
    fn from(from: Vec<Vec<&str>>) -> Self {
        IntoVec(
            from.into_iter()
                .map(|rest| rest.into_iter().map(ToOwned::to_owned).collect())
                .collect(),
        )
    }
}
impl<T> Deref for IntoVec<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
