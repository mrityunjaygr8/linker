use std::fmt::Debug;

use crate::link::Link;

#[derive(Debug, Default)]
pub struct Store {
    pub links: Vec<Link>,
}

impl Store {
    pub fn new() -> Self {
        Self { links: vec![] }
    }
    pub fn add(&mut self, link: &Link) {
        self.links.push(link.clone());
    }

    pub fn list(&self) -> &[Link] {
        return &self.links;
    }

    pub fn delete(&mut self, id: String) {
        for link in self.links.iter_mut() {
            if link.id == id {
                link.deleted = true;
                return;
            }
        }
    }

    pub fn get(&self, id: String) -> Option<&Link> {
        self.links.iter().find(|e| e.id == id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_store_creation() {
        let store = Store::new();
        assert_eq!(store.links, Vec::new());
    }

    #[test]
    fn test_store_link_add() {
        let mut store = Store::new();
        let link = Link::new("http://google.com").unwrap();
        store.add(&link);
        assert_eq!(store.links[0], link);
    }

    #[test]
    fn test_store_list() {
        let mut _store = Store::new();
        let _link = Link::new("http://google.com").unwrap();
    }
}
