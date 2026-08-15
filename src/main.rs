mod store;

use store::Store;
fn main() {
    let mut s_store = Store::new();

    let key = String::from("name");
    let value = String::from("Great");

    s_store.set(key, value);

    let name = s_store.get("name");

    if let Some(name) = name {
        println!("{name}")
    }
}