mod command;
mod error;
mod store;


use command::Command;
use store::Store;

fn main() {
    let mut store = Store::new();

    let put_name = Command::Put {
        key: "name".to_string(),
        value: "Yohan".to_string(),
    };

    let delete_project = Command::Delete {
        key: "project".to_string(),
    };

    match store.apply(put_name) {
        Ok(_) => println!("Put command applied succesfully"),
        Err(error) => println!("Error applying put command: {:?}", error),
    }

    match store.apply(delete_project) {
        Ok(_) => println!("Delete command applied succesfully"),
        Err(error) => println!("Error applying delete command: {:?}", error),
    }

    if let Some(value) = store.get("name") {
        println!("name = {}", value);
    } else {
        println!("name not found");
    }
}