use serde::{Serialize, de::DeserializeOwned};
use std::{
    collections::HashMap,
    fs::{self, File},
    io::{Read, Write},
    path::PathBuf,
};

pub struct KVLar {
    pub path: PathBuf,
}

impl KVLar {
    pub fn new(path: &str) -> Self {
        let path = PathBuf::from(path);

        // Cria o arquivo JSON vazio se não existir
        if !path.exists() {
            let _ = File::create(&path)
                .and_then(|mut f| f.write_all(b"{}"));
        }

        Self { path }
    }

    fn read_all(&self) -> std::io::Result<HashMap<String, serde_json::Value>> {
        let mut content = String::new();
        File::open(&self.path)?.read_to_string(&mut content)?;
        let map: HashMap<String, serde_json::Value> = serde_json::from_str(&content)?;
        Ok(map)
    }

    fn write_all(&self, map: &HashMap<String, serde_json::Value>) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(map)?;
        fs::write(&self.path, json)?;
        Ok(())
    }

    pub fn store<T: Serialize>(&self, key: &str, value: &T) -> std::io::Result<()> {
        let mut map = self.read_all()?;
        map.insert(key.to_string(), serde_json::to_value(value)?);
        self.write_all(&map)
    }

    pub fn get<T: DeserializeOwned>(&self, key: &str) -> std::io::Result<Option<T>> {
        let map = self.read_all()?;
        if let Some(val) = map.get(key) {
            let parsed = serde_json::from_value(val.clone())?;
            Ok(Some(parsed))
        } else {
            Ok(None)
        }
    }
}
