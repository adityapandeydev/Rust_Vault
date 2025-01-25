use crate::storage::Database;
use anyhow::Result;

pub enum Query {
    Insert { key: String, value: String },
    Get { key: String },
    Remove { key: String },
}

pub fn execute_query(db: &Database, query: Query) -> Result<()> {
    match query {
        Query::Insert { key, value } => {
            db.insert(key, value)?;
            println!("Inserted successfully.");
        }
        Query::Get { key } => {
            if let Some(value) = db.get(&key)? {
                println!("Value for {}: {}", key, value);
            } else {
                println!("No value found for {}", key);
            }
        }
        Query::Remove { key } => {
            if let Some(value) = db.remove(&key)? {
                println!("Removed value for {}: {}", key, value);
            } else {
                println!("No value found for {}", key);
            }
        }
    }
    Ok(())
}