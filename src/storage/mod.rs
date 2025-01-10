use std::collections::HashMap;

pub struct Storage {
    data: HashMap<String, String>,
}

impl Storage {
    pub fn new() -> Self {
        Storage {
            data: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }
}