use std::{env, process};

use projector::config::Config;

fn main() {
    let config = Config::build(&mut env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("{:?}", config);
}
