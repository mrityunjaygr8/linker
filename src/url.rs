use std::fmt::Display;

use url::Url;

#[derive(Debug, PartialEq)]
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

#[derive(Debug, Default, Clone, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_parse_correct() {
        let text = "http://google.com".to_string();
        let url = ParsedURL::parse(text.clone());
        let parsed = url.unwrap();
        assert_eq!(text, parsed.0)
    }

    #[test]
    fn test_url_parse_fail() {
        let text = "test.com".to_string();
        let url = ParsedURL::parse(text.clone());

        let err = url.unwrap_err();
        let expected_error = URLParseError::NotValidError(text.clone());
        assert_eq!(err, expected_error);
    }

    #[test]
    fn test_url_from_string() {
        let text = "http://test.com".to_string();
        let expected = ParsedURL { 0: text.clone() };

        let tested: ParsedURL = text.try_into().unwrap();
        assert_eq!(expected, tested);
    }

    #[test]
    fn test_url_parse_error_display() {
        let text = "test.com".to_string();
        let e = URLParseError::NotValidError(text.clone());
        assert_eq!(e.to_string(), format!("{}: URL is not valid", text))
    }
}
