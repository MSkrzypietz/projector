use std::{env, path::PathBuf};

#[derive(Debug)]
pub struct Config {
    pub operation: Operation,
    pub pwd: PathBuf,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let operation = match args.next() {
            Some(op) => op,
            None => return Err("No operation specified"),
        };

        let operation = match operation.as_ref() {
            "list" => Operation::List,
            "add" => {
                let key = match args.next() {
                    Some(key) => key,
                    None => return Err("No key specified"),
                };
                let value = match args.next() {
                    Some(value) => value,
                    None => return Err("No value specified"),
                };

                Operation::Add(key, value)
            }
            "rm" => {
                let key = match args.next() {
                    Some(key) => key,
                    None => return Err("No key specified"),
                };

                Operation::Remove(key)
            }
            _ => return Err("Unknown operation"),
        };

        let pwd = get_pwd()?;

        Ok(Config { operation, pwd })
    }
}

fn get_pwd() -> Result<PathBuf, &'static str> {
    match env::current_dir() {
        Ok(pwd) => Ok(pwd),
        Err(_) => Err("Unable to get current directory"),
    }
}

#[derive(Debug)]
pub enum Operation {
    List,
    Add(String, String),
    Remove(String),
}
