use super::model;
use crate::service::query_error::*;
use derive_more::*;

#[derive(new, Debug, AsRef)]
pub struct SaveGreetingRequest<'a> {
    value: &'a model::Greeting,
}

pub type SaveGreetingResult = Result<(), QueryError>;

#[cfg(test)]
use crate::component::*;
#[cfg(test)]
use async_trait::async_trait;

#[cfg(test)]
#[derive(new)]
pub struct StubRepository {
    save_greeting_result: SaveGreetingResult,
}

#[cfg(test)]
#[async_trait]
impl<'a, RQ: 'a> Component<'a, RQ, SaveGreetingResult> for StubRepository
where
    RQ: Send + Sync,
    &'a RQ: Into<SaveGreetingRequest<'a>>,
{
    async fn handle(&self, _: &'a RQ) -> SaveGreetingResult {
        self.save_greeting_result.clone()
    }
}
