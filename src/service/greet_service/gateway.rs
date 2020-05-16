use super::*;
use crate::component::*;
use crate::infrastructure::db;
use crate::schema::greetings;
use crate::service::query::{QueryError, QueryErrorKind, QueryResult};
use async_trait::async_trait;
use derive_new::*;
use diesel::prelude::*;
use tokio::task;

#[derive(new)]
pub struct Adaptor {
    tx: db::TransactionManager,
}

#[cfg(test)]
pub struct StubAdaptor {
    save_greeting_result: QueryResult<()>,
}

#[derive(Debug, Clone, PartialEq, Insertable)]
#[table_name = "greetings"]
struct GreetingRecord {
    id: uuid::Uuid,
    name: String,
    created_at: chrono::naive::NaiveDateTime,
}

impl From<&model::Greeting> for GreetingRecord {
    fn from(value: &model::Greeting) -> Self {
        Self {
            id: *(value.as_ref() as &model::Uuid).as_ref(),
            name: (value.as_ref() as &model::Name).as_ref().to_owned(),
            created_at: *(value.as_ref() as &model::Datetime).as_ref(),
        }
    }
}

#[async_trait]
impl Component<model::Greeting, QueryResult<()>> for Adaptor {
    async fn handle(&self, request: &model::Greeting) -> QueryResult<()> {
        let tx = self.tx.clone();
        let row = GreetingRecord::from(request);

        task::spawn_blocking(move || {
            tx.scope(|conn| {
                diesel::insert_into(greetings::table)
                    .values(row)
                    .execute(conn.as_ref() as &diesel::PgConnection)
                    .map(|_| ())
                    .map_err(Into::into)
            })
            .map_err(Into::into)
        })
        .await
        .map_err(|e| QueryError::new(QueryErrorKind::Other, e))
        .and_then(Into::into)
    }
}

#[cfg(test)]
#[async_trait]
impl Component<model::Greeting, QueryResult<()>> for StubAdaptor {
    async fn handle(&self, _: &model::Greeting) -> QueryResult<()> {
        self.save_greeting_result.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore]
    async fn insert_greeting_row() {
        let tx = db::get_test_transaction_manager();
        let adaptor = Adaptor::new(tx);
        let greeting = model::Greeting::try_new("foo").unwrap();
        let result = adaptor.handle(&greeting).await;
        assert!(result.is_ok())
    }
}
