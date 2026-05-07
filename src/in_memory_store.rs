use std::fmt::Debug;

use crate::link::Link;

/// Store is an in memory store for holding [crate::link::Link] vectors
#[derive(Debug, Default)]
pub struct Store {
    pub links: Vec<Link>,
}

impl Store {
    /// Create a new store instance.  
    /// Example
    /// ```
    /// let a = linker::Store::new();
    /// ```
    pub fn new() -> Self {
        Self { links: vec![] }
    }
    /// Add a link to the store.  
    /// Example
    /// ```
    /// let l = linker::Link::new("http://google.com").unwrap();
    /// let mut store = linker::Store::new();
    /// store.add(&l);
    /// ```
    pub fn add(&mut self, link: &Link) {
        self.links.push(link.clone());
    }

    /// List the links stored.  
    /// Deleted links will not be returned.  
    /// Example
    /// ```
    /// let l = linker::Link::new("http://google.com").unwrap();
    /// let mut store = linker::Store::new();
    /// store.add(&l);
    /// let list = store.list();
    /// ```
    pub fn list(&self) -> Vec<&Link> {
        return self.links.iter().filter(|l| l.deleted == false).collect();
    }

    /// If the link with ID exists, its deleted value will be set to true.  
    /// If the link with ID does not exists, will not raise an error.  
    /// Example
    /// ```
    /// let l = linker::Link::new("http://google.com").unwrap();
    /// let mut store = linker::Store::new();
    /// store.add(&l);
    /// let id = l.id;
    /// store.delete(id);
    /// store.delete("does not exist".to_string());
    /// ```
    pub fn delete(&mut self, id: String) {
        for link in self.links.iter_mut() {
            if link.id == id {
                link.deleted = true;
                return;
            }
        }
    }

    /// If the link with ID exists, it will be returned.  
    /// If the link has been deleted, None will be returned.  
    /// If the link does not exist, None will be returned.  
    /// Example
    /// ```
    /// let l = linker::Link::new("http://google.com").unwrap();
    /// let mut store = linker::Store::new();
    /// store.add(&l);
    /// let id = l.id.clone();
    /// assert_eq!(Some(&l), store.get(id.clone()));
    /// store.delete(id.clone());
    /// assert_eq!(None, store.get(id));
    /// assert_eq!(None, store.get("woo-123".to_string()));
    ///
    /// ```
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
        println!("{}", store.links.len());
        let list = store.list();
        println!("Size: {}", store.list().len());
        assert_eq!(list, [&link]);
    }
    #[test]
    fn test_store_list_deleted() {
        let mut store = Store::new();
        let link = Link::new("http://google.com").unwrap();

        store.add(&link);
        let id = store.links.first().unwrap().id.clone();

        store.delete(id);
        assert_eq!(store.links.first().unwrap().deleted, true);
        let list = store.list();
        assert_eq!(list.len(), 0);
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
