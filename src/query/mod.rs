use crate::storage::Storage;

pub fn execute_query(query: &str, storage: &mut Storage) {
    if query.starts_with("INSERT") {
        let parts: Vec<&str> = query.split_whitespace().collect();
        let key = parts[1].to_string();
        let value = parts[2].to_string();
        storage.insert(key, value);
    } else if query.starts_with("SELECT") {
        let parts: Vec<&str> = query.split_whitespace().collect();
        let key = parts[1];
        match storage.get(key) {
            Some(value) => println!("Value: {}", value),
            None => println!("No value found for key: {}", key),
        }
    }
}
