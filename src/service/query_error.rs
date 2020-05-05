use crate::infrastructure::db::DbError;
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

impl From<DbError> for QueryError {
    fn from(value: DbError) -> Self {
        match value {
            DbError::Connection(e) => QueryError::new(QueryErrorKind::FailedToConnectStore, e),
            DbError::Handling(e) => QueryError::new(
                match &e {
                    diesel::result::Error::InvalidCString(_)
                    | diesel::result::Error::QueryBuilderError(_)
                    | diesel::result::Error::DeserializationError(_)
                    | diesel::result::Error::SerializationError(_) => {
                        QueryErrorKind::FailedToConstructRequest
                    }
                    diesel::result::Error::DatabaseError(_, _) => QueryErrorKind::InvalidRequest,
                    _ => QueryErrorKind::Other,
                },
                e,
            ),
        }
    }
}
