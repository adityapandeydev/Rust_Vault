mod storage;
mod query;
mod utils;

use anyhow::Result;
use storage::Database;

fn main() -> Result<()> {
    // Initialize the database
    let mut db = Database::new();

    // Insert some data
    db.insert("key1".to_string(), "value1".to_string())?;
    db.insert("key2".to_string(), "value2".to_string())?;

    // Query data
    if let Some(value) = db.get("key1")? {
        println!("Found value for key1: {}", value);
    } else {
        println!("No value found for key1");
    }

    // Save the database to disk
    db.save("database.json")?;

    // Load the database from disk
    let loaded_db = Database::load("database.json")?;
    if let Some(value) = loaded_db.get("key1")? {
        println!("Loaded value for key1: {}", value);
    }

    Ok(())
}