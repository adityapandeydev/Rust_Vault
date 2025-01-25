use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

#[derive(Serialize, Deserialize)]
struct DatabaseData {
    data: HashMap<String, String>,
}

#[derive(Clone)]
pub struct Database {
    data: Arc<Mutex<HashMap<String, String>>>,
}

impl Database {
    pub fn new() -> Self {
        Database {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn insert(&self, key: String, value: String) -> Result<(), StorageError> {
        let mut data = self.data.lock().unwrap();
        data.insert(key, value);
        Ok(())
    }

    pub fn get(&self, key: &str) -> Result<Option<String>, StorageError> {
        let data = self.data.lock().unwrap();
        Ok(data.get(key).cloned())
    }

    pub fn remove(&self, key: &str) -> Result<Option<String>, StorageError> {
        let mut data = self.data.lock().unwrap();
        Ok(data.remove(key))
    }

    pub fn save(&self, path: &str) -> Result<(), StorageError> {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        let data = self.data.lock().unwrap();
        let db_data = DatabaseData {
            data: data.clone(),
        };
        serde_json::to_writer(writer, &db_data)?;
        Ok(())
    }

    pub fn load(path: &str) -> Result<Self, StorageError> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let db_data: DatabaseData = serde_json::from_reader(reader)?;
        Ok(Database {
            data: Arc::new(Mutex::new(db_data.data)),
        })
    }
}