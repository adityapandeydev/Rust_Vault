use Rust_Vault::storage::Storage;
use Rust_Vault::query::execute_query;
use Rust_Vault::utils::log_message;

fn main() {
    let mut storage = Storage::new();
    let query = "INSERT key1 value1"; // Example query

    log_message("Executing query...");
    execute_query(query, &mut storage);

    let select_query = "SELECT key1";
    execute_query(select_query, &mut storage);
}
