mod store;

use store::Store;
fn main() {
    let mut s_store = Store::new();

    let key = String::from("name");
    let value = String::from("Great");

    s_store.set(key, value);
}