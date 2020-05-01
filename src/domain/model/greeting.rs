use super::*;
use anyhow::*;
use std::convert::TryFrom;

#[derive(Debug, Clone, PartialEq)]
pub struct Greeting {
    id: Uuid,
    name: Name,
    created_at: Datetime,
}

impl Greeting {
    pub fn try_new(name: impl Into<String>) -> Result<Self> {
        let id = Uuid::new();
        let name =
            Name::try_from(name.into().as_str()).with_context(|| "failed to convert name")?;
        let created_at = Datetime::now();

        Ok(Self {
            id,
            name,
            created_at,
        })
    }
}

impl Greet for Greeting {
    fn hello(&self) -> Message {
        Message::new("hello", self.name.as_ref(), "🎉")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_new_is_ok() {
        let actual = Greeting::try_new("foo");
        assert!(actual.is_ok())
    }

    #[test]
    fn try_new_is_err() {
        let actual = Greeting::try_new((0..10000).into_iter().map(|_| "_").collect::<String>());
        assert!(actual.is_err())
    }
}
