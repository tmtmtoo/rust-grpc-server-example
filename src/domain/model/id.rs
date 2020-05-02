use anyhow::*;
use derive_more::*;
use std::convert::TryFrom;

#[derive(Debug, Clone, PartialEq, AsRef)]
pub struct Uuid(uuid::Uuid);

impl Uuid {
    pub fn random() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}

impl TryFrom<&str> for Uuid {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let uuid = uuid::Uuid::parse_str(value).with_context(|| "failed to parse uuid string")?;
        Ok(Self(uuid))
    }
}

impl ToString for Uuid {
    fn to_string(&self) -> String {
        self.0.to_hyphenated().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_from_is_ok() {
        let value = "eaefc36e-6f4f-4b62-8efb-2da86fbf1d9f";
        let actual = Uuid::try_from(value);
        assert!(actual.is_ok())
    }

    #[test]
    fn try_from_is_err() {
        let value = "foo";
        let actual = Uuid::try_from(value);
        assert!(actual.is_err())
    }

    #[test]
    fn new_as_uuid() {
        let id = Uuid::random();
        let uuid_string = id.to_string();
        let actual = uuid::Uuid::parse_str(uuid_string.as_str());
        assert!(actual.is_ok())
    }
}
