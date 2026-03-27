mod store;
mod log;
mod error;
mod command;

use command::Command;
use log::ReplicatedLog;
use store::Store;

fn main() {
    let mut store = Store::new();
    let mut log = ReplicatedLog::new();

    let index1 = log.append(Command::Put {
        key: "Name".to_string(),
        value: "Yohan".to_string(),
    });

    let index2 = log.append(Command::Put {
        key: "Project".to_string(),
        value: "Apex".to_string(),
    });

    let index3 = log.append(Command::Delete {
        key: "Project".to_string(),
    });

    println!("Append entries at index: {}, {}, {}", index1, index2, index3);

    for entry in log.entries() {
        match store.apply(entry.command.clone()) {
            Ok(_) => println!("Applied entry at index {}", entry.index),
            Err(error) => println! ("Failed to apply entry at index {}: {:?}", entry.index, error),
        }
    }

    if let Some(value) = store.get("Name") {
        println!("Name = {}", value);
    } else {
        println!("Name was not found")
    }

    if let Some(value) = store.get("Project") {
        println!("Project = {}", value);
    } else {
        println!("Project was not found")
    }
}