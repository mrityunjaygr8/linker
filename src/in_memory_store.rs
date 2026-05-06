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
        self.links.iter().find(|e| e.id == id && e.deleted == false)
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
        let mut store = Store::new();
        let link = Link::new("http://google.com").unwrap();

        store.add(&link);
        let list = store.list();
        assert_eq!(list, [link]);
    }

    #[test]
    fn test_store_get() {
        let mut store = Store::new();
        let link = Link::new("http://google.com").unwrap();

        store.add(&link);
        let id = store.links.first().unwrap().id.clone();
        let got = store.get(id).unwrap();
        assert_eq!(got, &link);
    }
    #[test]
    fn test_store_get_deleted_none() {
        let mut store = Store::new();
        let link = Link::new("http://google.com").unwrap();

        store.add(&link);
        let id = store.links.first().unwrap().id.clone();
        let got = store.get(id.clone()).unwrap();
        assert_eq!(got, &link);
        store.delete(id.clone());
        let got = store.get(id);
        assert_eq!(got, None);
    }
    #[test]
    fn test_store_get_miss() {
        let mut store = Store::new();
        let link = Link::new("http://google.com").unwrap();

        store.add(&link);
        let id = "woo-123".to_string();
        let got = store.get(id);
        assert_eq!(got, None);
    }

    #[test]
    fn test_store_delete() {
        let mut store = Store::new();
        let link = Link::new("http://google.com").unwrap();

        store.add(&link);
        let id = store.links.first().unwrap().id.clone();

        store.delete(id);
        assert_eq!(store.links.first().unwrap().deleted, true)
    }
    #[test]
    fn test_store_delete_missing() {
        let mut store = Store::new();
        let link = Link::new("http://google.com").unwrap();

        store.add(&link);
        let id = "woo-123".to_string();

        store.delete(id);
    }
}
