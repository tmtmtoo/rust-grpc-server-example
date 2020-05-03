use anyhow::*;
use derive_more::{AsRef, Display};
use std::sync::Arc;
use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Display)]
pub enum QueryErrorKind {
    FailedToConnectStore,
    FailedToConstructRequest,
    InvalidRequest,
    InvalidResponse,
    Aborted,
    Other,
}

#[derive(Error, Display, Debug, Clone, AsRef)]
#[display(fmt = "{}: {}", kind, source)]
pub struct QueryError {
    #[as_ref]
    kind: QueryErrorKind,
    #[as_ref]
    source: Arc<Error>,
}

impl QueryError {
    pub fn new(kind: QueryErrorKind, source: impl Into<Error>) -> Self {
        Self {
            kind,
            source: Arc::new(source.into()),
        }
    }
}

impl PartialEq for QueryError {
    fn eq(&self, other: &Self) -> bool {
        self.kind.eq(&other.kind)
    }
}

#[derive(new, Debug)]
pub struct SaveRequest<'a, T> {
    value: &'a T,
}

impl<'a, T> AsRef<T> for SaveRequest<'a, T> {
    fn as_ref(&self) -> &T {
        self.value
    }
}

pub type SaveResult<T> = Result<T, QueryError>;

#[cfg(test)]
use crate::component::*;
#[cfg(test)]
use crate::domain::model::*;
#[cfg(test)]
use async_trait::async_trait;

#[cfg(test)]
#[derive(new)]
pub struct StubRepository {
    save_greeting_result: SaveResult<()>,
}

#[cfg(test)]
#[async_trait]
impl<'a, RQ: 'a> Component<'a, RQ, SaveResult<()>> for StubRepository
where
    RQ: Send + Sync,
    &'a RQ: Into<SaveRequest<'a, Greeting>>,
{
    async fn handle(&self, _: &'a RQ) -> SaveResult<()> {
        self.save_greeting_result.clone()
    }
}
