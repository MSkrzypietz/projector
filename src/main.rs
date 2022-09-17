use std::{env, process};

use projector::{
    config::{Config, Operation},
    projector::Projector,
};

fn main() {
    let config = Config::build(&mut env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let mut projector = Projector::from_config(config.pwd, config.storage);
    match config.operation {
        Operation::List(Some(key)) => match projector.get_value(&key) {
            Some(val) => println!("Found entry: {key} => {val}"),
            None => println!("Not found"),
        },
        Operation::List(None) => {
            let all_values = projector.get_all_values();
            println!("{:?}", all_values);
        }
        Operation::Add(key, value) => {
            projector.add(key.clone(), value.clone());
            match projector.save() {
                Ok(_) => println!("Added: {key} => {value}"),
                Err(e) => eprintln!("Error adding {key} => {value}: {e}"),
            }
        }
        Operation::Remove(key) => {
            projector.remove(&key);
            match projector.save() {
                Ok(_) => println!("Removed: {key}"),
                Err(e) => eprintln!("Error removing {key}: {e}"),
            }
        }
    }
}
