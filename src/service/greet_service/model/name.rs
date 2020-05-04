use anyhow::*;
use boolinator::*;
use derive_more::*;
use std::convert::TryFrom;

#[derive(Debug, Clone, PartialEq, AsRef)]
pub struct Name(String);

impl Name {
    const MAX_LENGTH: usize = 255;
}

impl TryFrom<&str> for Name {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        (value.len() <= Self::MAX_LENGTH).ok_or_else(|| Error::msg("too long"))?;
        Ok(Self(value.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_from_is_ok() {
        let actual = Name::try_from("foo");
        assert!(actual.is_ok())
    }

    #[test]
    fn try_from_is_ok_max() {
        let actual = Name::try_from(
            (0..Name::MAX_LENGTH)
                .into_iter()
                .map(|_| "_")
                .collect::<String>()
                .as_str(),
        );
        assert!(actual.is_ok())
    }

    #[test]
    fn try_from_is_err() {
        let actual = Name::try_from(
            (0..Name::MAX_LENGTH + 1)
                .into_iter()
                .map(|_| "_")
                .collect::<String>()
                .as_str(),
        );
        assert!(actual.is_err())
    }
}
