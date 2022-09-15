use std::{env, process};

use projector::{config::Config, projector::Projector};

fn main() {
    let config = Config::build(&mut env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("{:?}", config);

    let mut projector = Projector::from_config(config);
    if let Err(err) = projector.execute() {
        eprintln!("Problem executing the operation: {}", err);
        process::exit(1);
    }

    println!("{:?}", projector);
    println!("Success");
}
