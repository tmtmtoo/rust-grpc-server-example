#[cfg(test)]
use super::model;
#[cfg(test)]
use crate::component::*;
#[cfg(test)]
use crate::service::query_error::*;
#[cfg(test)]
use async_trait::async_trait;

#[cfg(test)]
#[derive(new)]
pub struct StubRepository {
    save_greeting_result: QueryResult<()>,
}

#[cfg(test)]
#[async_trait]
impl<'a> Component<model::Greeting, QueryResult<()>> for StubRepository {
    async fn handle(&self, _: &model::Greeting) -> QueryResult<()> {
        self.save_greeting_result.clone()
    }
}
