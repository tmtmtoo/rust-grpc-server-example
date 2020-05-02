use derive_more::*;

#[derive(Debug, Clone, PartialEq, From, AsRef)]
pub struct Datetime(chrono::NaiveDateTime);

impl Datetime {
    pub fn now() -> Self {
        Self(chrono::Utc::now().naive_utc())
    }
}
