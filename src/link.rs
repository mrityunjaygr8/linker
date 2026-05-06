use std::fmt::Display;

use chrono::{DateTime, Utc};

use crate::{
    id,
    url::{ParsedURL, URLParseError},
};

#[derive(Debug, PartialEq)]
pub enum LinkError {
    URLError(URLParseError),
}

impl Display for LinkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LinkError::URLError(e) => write!(f, "{}", e),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Link {
    pub id: String,
    pub url: ParsedURL,
    _created: DateTime<Utc>,
    pub deleted: bool,
}

impl Display for Link {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.id, self.url)
    }
}

impl Link {
    pub fn new(url: String) -> Result<Self, LinkError> {
        let url_parsed = match url.try_into() {
            Ok(u) => u,
            Err(e) => return Err(LinkError::URLError(e)),
        };
        Ok(Link {
            id: id::generate(),
            url: url_parsed,
            _created: chrono::Utc::now(),
            deleted: false,
        })
    }
}

#[cfg(test)]
mod tests {
    use chrono::Duration;

    use super::*;

    #[test]
    fn test_link_creation() {
        let text = "http://google.com".to_string();
        let link = Link::new(text.clone()).unwrap();
        let url = ParsedURL::parse(text.clone()).unwrap();

        assert_eq!(link.deleted, false);
        assert_eq!(link.url, url);
        assert_eq!(link.id.len(), 10);
        // Asserts that the time delta between `link._created` and now is less than 1 second
        assert_eq!(
            Utc::now() - link._created < Duration::new(1, 0).unwrap(),
            true
        );
    }

    #[test]
    fn test_link_invalid() {
        let text = "google.com".to_string();
        let link_err = Link::new(text.clone()).unwrap_err();
        assert_eq!(
            link_err,
            LinkError::URLError(URLParseError::NotValidError(text))
        );
    }

    #[test]
    fn test_link_error_display() {
        let text = "google.com".to_string();
        let err = LinkError::URLError(URLParseError::NotValidError(text.clone()));
        assert_eq!(err.to_string(), format!("{}: URL is not valid", text));
    }

    #[test]
    fn test_link_display() {
        let text = "http://google.com".to_string();
        let link = Link::new(text.clone()).unwrap();
        let binding = link.to_string();
        let mut formatted = binding.split(": ");
        let id_part = formatted.next().unwrap();
        assert_eq!(id_part.len(), 10);
        let next = formatted.next().unwrap();
        assert_eq!(next, text);
    }
}
