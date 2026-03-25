mod command;
mod store;

use command::Command;
use store::Store;

fn main(){
    let mut store = Store::new();

    let put_name = Command::Put{
        key: "name".to_string(),
        value: "Yohan".to_string(),
    };

    let put_project = Command::Put{
        key: "project".to_string(),
        value: "Apex".to_string(),
    };

    let delete_project = Command::Delete{
        key: "project".to_string(),
    };

    store.apply(put_name);
    store.apply(put_project);

    if let Some(value) = store.get("name") {
        println!("name = {}", value)
    } else {
        println!("name not found")
    }

    store.apply(delete_project);

    if let Some(value) = store.get("project") {
        println!("project = {}", value)
    } else {
        println!("project not found")
    }
}