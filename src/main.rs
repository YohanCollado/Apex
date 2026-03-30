mod command;
mod error;
mod log;
mod node;
mod store;

use command::Command;
use node::ApexNode;

fn main() {
    let mut node = ApexNode::new();

    match node.apply_command(Command::Put {
        key: "name".to_string(),
        value: "Yohan".to_string(),
    }) {
        Ok(index) => println!("Put command appended and applied at log index {}", index),
        Err(error) => println!("Error applying put command: {:?}", error),
    }

    match node.apply_command(Command::Put {
        key: "project".to_string(),
        value: "Apex".to_string(),
    }) {
        Ok(index) => println!("Put command appended and applied at log index {}", index),
        Err(error) => println!("Error applying put command: {:?}", error),
    }

    match node.apply_command(Command::Delete {
        key: "project".to_string(),
    }) {
        Ok(index) => println!("Delete command appended and applied at log index {}.", index),
        Err(error) => println!("Error applying delete command: {:?}", error),
    }

    if let Some(value) = node.get("name") {
        println!("Name = {}", value);
    } else {
        println!("Name not found")
    }
        
    if let Some(value) = node.get("project") {
        println!("Project = {}", value);
    } else {
        println!("Project was not found")
    }

    println!("Last log index = {}", node.last_log_index());
    println!("Log: {:?}", node.get_log());
}