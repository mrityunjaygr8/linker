use std::fmt::Display;

use url::Url;

#[derive(Debug)]
pub enum URLParseError {
    NotValidError(String),
}

impl Display for URLParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            URLParseError::NotValidError(e) => write!(f, "{}: URL is not valid", e),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct ParsedURL(String);
impl ParsedURL {
    pub fn parse(value: String) -> Result<Self, URLParseError> {
        Url::parse(&value)
            .map_err(|_| URLParseError::NotValidError(value.clone()))
            .map(|_| Self(value))
    }
}

impl Display for ParsedURL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<String> for ParsedURL {
    type Error = URLParseError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        ParsedURL::parse(value)
    }
}
