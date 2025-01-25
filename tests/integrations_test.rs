use rust_vault::storage::Database;

#[test]
fn test_database_insert_and_get() {
    let db = Database::new();
    db.insert("key1".to_string(), "value1".to_string()).unwrap();

    let value = db.get("key1").unwrap();
    assert_eq!(value, Some("value1".to_string()));
}

#[test]
fn test_database_remove() {
    let db = Database::new();
    db.insert("key1".to_string(), "value1".to_string()).unwrap();

    db.remove("key1").unwrap();
    let value = db.get("key1").unwrap();
    assert_eq!(value, None);
}