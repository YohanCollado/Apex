mod command;
mod error;
mod log;
mod node;
mod store;

use node::ApexNode;
use std::io::{self, Write};

fn main() {
    let mut node = ApexNode::new();
    let mut input = String::new();

    println!("Apex");
    
    loop {
        input.clear();

        print!("> ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut input).unwrap();

        let command = input.trim().to_lowercase();

        if command == "exit"{
            println!("Bye Bye... See you next time!");
            break;
        }
        if command.is_empty() {
            println!("Please enter a command. Empty commands are not accepted");
            continue;
        }
        
        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        let cmd = match parts[0] {
            "put" => {
                if parts.len() < 3 {
                    println!("Usage: put <key> <value>");
                    continue;
                }

                command::Command::Put { 
                    key: parts[1].to_string(), 
                    value: parts[2..].join(" "), 
                }
            }

            "get" => {
                if parts.len() < 2 {
                    println!("Usage: get <key>");
                    continue;
                }
                command::Command::Get{
                    key: parts[1].to_string(),
                }
            }

            "delete" => {
                if parts.len() < 2 {
                    println!("Usage: delete <key>");
                    continue;
                }
                command::Command::Delete { 
                    key: parts[1].to_string() 
                }
            }
            _ => {
                println!("Unkown command. Please try again!");
                continue;
            }
        };
        
        match cmd {
            command::Command::Get { key } => {
                match node.get(&key) {
                    Some(value) => println! ("Value: {}", value),
                    None => println!("Error: KeyNotFound({})", key)
                }
            }
            _ => {
                match node.apply_command(cmd) {
                    Ok((index, Some(value))) => {
                        println!("Log Index: {}", index);
                        println!("Value: {}", value);
                    }
                    Ok((index, None)) => {
                        println!("Applied successfully at log index {}", index);
                    }
                    Err(error) => {
                        println!("Error: {:?}", error)
                    }
                }
            }
        }
    }
}
