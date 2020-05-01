use anyhow::*;
use std::sync::Arc;
use thiserror::Error as ThisError;

#[derive(ThisError, Clone, Debug, PartialEq)]
pub enum QueryErrorKind {
    #[error("failed to connect store")]
    FailedToConnectStore,
    #[error("failed to construct request")]
    FailedToConstructRequest,
    #[error("unauthorized request")]
    UnauthorizedRequest,
    #[error("invalid request")]
    InvalidRequest,
    #[error("invalid response")]
    InvalidResponse,
    #[error("connection aborted")]
    Aborted,
    #[error("unknown storing error")]
    Other,
}

#[derive(ThisError, Debug, Clone)]
pub struct QueryError {
    kind: QueryErrorKind,
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

impl std::fmt::Display for QueryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.kind, self.source)
    }
}

impl PartialEq for QueryError {
    fn eq(&self, other: &Self) -> bool {
        self.kind.eq(&other.kind)
    }
}

impl AsRef<Error> for QueryError {
    fn as_ref(&self) -> &Error {
        &*self.source
    }
}

impl AsRef<QueryErrorKind> for QueryError {
    fn as_ref(&self) -> &QueryErrorKind {
        &self.kind
    }
}

pub struct SaveRequest<T> {
    value: T,
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
impl Component<SaveRequest<Greeting>, SaveResult<()>> for StubRepository {
    async fn handle(&self, _: &SaveRequest<Greeting>) -> SaveResult<()> {
        self.save_greeting_result.clone()
    }
}
