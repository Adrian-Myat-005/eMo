use std::collections::HashMap;
use crate::types::Value;

pub struct Backpack {
    storage: HashMap<String, Value>,
}

impl Backpack {
    pub fn new() -> Self {
        Backpack {
            storage: HashMap::new(),
        }
    }

    pub fn hold(&mut self, name: String, value: Value) {
        self.storage.insert(name, value);
    }

    pub fn release(&self, name: &str) -> Option<Value> {
        self.storage.get(name).cloned()
    }
}