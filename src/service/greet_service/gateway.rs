use super::*;
use crate::component::*;
use crate::infrastructure::db;
use crate::schema::greetings;
use crate::service::query_error::*;
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
    save_greeting_result: super::SaveGreetingResult,
}

#[derive(Debug, Clone, PartialEq, Insertable)]
#[table_name = "greetings"]
struct GreetingRecord {
    id: uuid::Uuid,
    name: String,
    created_at: chrono::naive::NaiveDateTime,
}

impl<'a> From<&'a SaveGreetingRequest<'a>> for GreetingRecord {
    fn from(value: &'a SaveGreetingRequest<'a>) -> Self {
        let greeting = value.as_ref();
        Self {
            id: *(greeting.as_ref() as &model::Uuid).as_ref(),
            name: (greeting.as_ref() as &model::Name).as_ref().to_owned(),
            created_at: *(greeting.as_ref() as &model::Datetime).as_ref(),
        }
    }
}

#[async_trait]
impl<'a, RQ: 'a> Component<'a, RQ, SaveGreetingResult> for Adaptor
where
    RQ: Send + Sync,
    &'a RQ: Into<SaveGreetingRequest<'a>>,
{
    async fn handle(&self, request: &'a RQ) -> SaveGreetingResult {
        let tx = self.tx.clone();
        let row = GreetingRecord::from(&request.into());

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
impl<'a, RQ: 'a> Component<'a, RQ, SaveGreetingResult> for StubAdaptor
where
    RQ: Send + Sync,
    &'a RQ: Into<SaveGreetingRequest<'a>>,
{
    async fn handle(&self, _: &'a RQ) -> SaveGreetingResult {
        self.save_greeting_result.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl<'a> Into<SaveGreetingRequest<'a>> for &'a SaveGreetingRequest<'a> {
        fn into(self) -> SaveGreetingRequest<'a> {
            SaveGreetingRequest::new(self.as_ref())
        }
    }

    #[tokio::test]
    #[ignore]
    async fn insert_greeting_row() {
        let tx = db::get_test_transaction_manager();
        let adaptor = Adaptor::new(tx);
        let greeting = model::Greeting::try_new("foo").unwrap();
        let request = SaveGreetingRequest::new(&greeting);
        let result = adaptor.handle(&request).await;
        assert!(result.is_ok())
    }
}
