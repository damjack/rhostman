use std::fmt::Formatter;
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Validate, Deserialize)]
pub struct Host {
    #[validate(url)]
    url: String,
}

impl From<String> for Host {
    fn from(host: String) -> Self {
        Self { url: host }
    }
}

impl std::fmt::Display for Host {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.url)
    }
}
