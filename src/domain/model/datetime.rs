#[derive(Debug, Clone, PartialEq)]
pub struct Datetime(chrono::NaiveDateTime);

impl Datetime {
    pub fn now() -> Self {
        Self(chrono::Utc::now().naive_utc())
    }
}

impl From<chrono::NaiveDateTime> for Datetime {
    fn from(value: chrono::NaiveDateTime) -> Self {
        Self(value)
    }
}

impl AsRef<chrono::NaiveDateTime> for Datetime {
    fn as_ref(&self) -> &chrono::NaiveDateTime {
        &self.0
    }
}
