use std::fmt::Display;

use chrono::{DateTime, Utc};

use crate::{
    id,
    url::{ParsedURL, URLParseError},
};

#[derive(Debug)]
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
