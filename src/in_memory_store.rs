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
    pub fn add(&mut self, link: Link) -> &Self {
        self.links.push(link);
        self
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
