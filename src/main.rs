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
        Operation::List(Some(key)) => {
            match projector.get_value(&key) {
                Some(val) => println!("{key}: {val}"),
                None => println!("Not found")
            }
        }
        Operation::List(None) => {
            let res = projector.get_all_values();
            println!("{:?}", res);
        }
        Operation::Add(key, value) => {
            projector.add(key.clone(), value.clone());
            projector.save();
            println!("Added {} with {}", key, value);
        }
        Operation::Remove(key) => {
            projector.remove(&key);
            projector.save();
            println!("Removed {}", key); 
        }
    }
}
