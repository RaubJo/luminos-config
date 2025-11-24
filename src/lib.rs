use std::collections::HashMap;
mod env;
mod node;
mod codegen;

pub use env::env;
pub use node::Node;
pub use crate::codegen::generate;
pub use dotenv::dotenv;


#[derive(Debug, Clone, Default)]
pub struct Config {
    pub root: Node,
}

impl Config {
    pub fn new() -> Self {
        Self {
            root: Node::Object(HashMap::new()),
        }
    }

    /// Insert a section.
    pub fn insert_section(&mut self, name: &str, entries: &[(&str, Node)]) {
        let mut map = HashMap::new();
        for (k, v) in entries {
            map.insert((*k).to_string(), v.clone());
        }

        if let Node::Object(root) = &mut self.root {
            root.insert(name.to_string(), Node::Object(map));
        }
    }

    /// Get a value by key
    pub fn get(&self, path: &str) -> Option<&Node> {
        let mut cur = &self.root;
        for key in path.split('.') {
            match cur {
                Node::Object(map) => cur = map.get(key)?,
                _ => return None,
            }
        }
        Some(cur)
    }

    /// Set a config value by key 
    pub fn set(&mut self, path: &str, value: Node) {
        let mut cur = &mut self.root;
        for key in path.split('.') {
            cur = match cur {
                Node::Object(map) => {
                    map.entry(key.to_string())
                        .or_insert(Node::Object(HashMap::new()))
                }
                _ => panic!("Cannot set into non-object"),
            };
        }
        *cur = value;
    }
}
