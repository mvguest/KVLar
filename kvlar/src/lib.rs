use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Write};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct KVLar {
    store: HashMap<String, String>,
}

impl KVLar {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.store.get(key)
    }

    pub fn save(&self, path: &str) -> std::io::Result<()> {
        let serialized = serde_json::to_string_pretty(&self)?;
        let mut file = File::create(path)?;
        file.write_all(serialized.as_bytes())?;
        Ok(())
    }

    pub fn load(path: &str) -> std::io::Result<Self> {
        let file = OpenOptions::new().read(true).open(path)?;
        let reader = BufReader::new(file);
        let store: KVLar = serde_json::from_reader(reader)?;
        Ok(store)
    }
}
