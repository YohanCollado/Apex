mod store;

use store::Store;

fn main() {
    let mut store = Store::new();

    store.put("name".to_string(), "Yohan".to_string());
    store.put("project".to_string(), "Apex".to_string());

    if let Some(value) = store.get("name"){
        println!("name = {}", value);
    } else {
        println!("name not found");
    }

    //store.delete("project");

    if let Some(value) = store.get("project") {
        println!("project = {}", value);
    } else {
        println!("project not found")
    }
}