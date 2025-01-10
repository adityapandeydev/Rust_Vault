use Rust_Vault::storage::Storage;
use Rust_Vault::query::execute_query;

#[test]
fn test_insert_and_select() {
    let mut storage = Storage::new();
    execute_query("INSERT key1 value1", &mut storage);
    execute_query("SELECT key1", &mut storage);
    assert_eq!(storage.get("key1"), Some(&"value1".to_string()));
}
