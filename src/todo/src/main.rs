use bincode::config::standard;
use bincode::{Decode, Encode};

use sled::Db; // embedded key-value store
use std::env;

#[derive(Debug, Encode, Decode)]
enum Prior {
    Fuck,   // Custom priority levels
    High,
    Medium,
    Soft,
    Chill,
}

#[derive(Debug, Encode, Decode)]
struct Todo {
    thing: String,
    priority: Prior,
    done: bool,
}

fn main() {
    // Example usage of serialization/deserialization (commented out)
    // let todo = Todo::new("Studiare meglio rust".into(), Prior::Fuck);
    // let encoded = Todo::serialize(&todo);
    // let decoded = Todo::deserialize(&encoded);
    // println!("Original: {:#?}", decoded);
    // println!("Modified {:#?}", encoded);

    let args: Vec<String> = env::args().collect(); // Collect command-line arguments

    if args.len() > 1 {
        let command = &args[1];
        match &command[..] {
            _ => println!("{:#?}", &args[2..]), // Just prints the remaining arguments for now
        }
    } else {
        // todo.list(); // Placeholder for future implementation
    }
}

impl Todo {
    fn new(thing: String, priority: Prior) -> Self {
        Self {
            thing,
            priority,
            done: false, // Defaults to not done
        }
    }

    fn serialize(listed: &Self) -> Vec<u8> {
        bincode::encode_to_vec(listed, standard()).unwrap() // Serializes the Todo to bytes
    }

    fn deserialize(encoded: &[u8]) -> Self {
        let (decoded, _) = bincode::decode_from_slice(encoded, standard()).unwrap(); // Deserializes back to a Todo
        decoded
    }
}
