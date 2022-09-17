use std::{env, path::PathBuf};

use anyhow::{anyhow, Result};

#[derive(Debug)]
pub enum Operation {
    List(Option<String>),
    Add(String, String),
    Remove(String),
}

#[derive(Debug)]
pub struct Config {
    pub operation: Operation,
    pub pwd: PathBuf,
    pub storage: PathBuf,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config> {
        args.next();

        let operation = match args.next() {
            Some(op) => op,
            None => return Err(anyhow!("No operation specified")),
        };

        let operation = match operation.as_ref() {
            "list" => match args.next() {
                Some(key) => Operation::List(Some(key)),
                None => Operation::List(None),
            },
            "add" => {
                let key = match args.next() {
                    Some(key) => key,
                    None => return Err(anyhow!("No key specified")),
                };
                let value = match args.next() {
                    Some(value) => value,
                    None => return Err(anyhow!("No value specified")),
                };

                Operation::Add(key, value)
            }
            "rm" => {
                let key = match args.next() {
                    Some(key) => key,
                    None => return Err(anyhow!("No key specified")),
                };

                Operation::Remove(key)
            }
            _ => return Err(anyhow!("Unknown operation")),
        };

        Ok(Config {
            operation,
            pwd: Self::get_pwd()?,
            storage: Self::get_storage_path()?,
        })
    }

    fn get_pwd() -> Result<PathBuf> {
        match env::current_dir() {
            Ok(pwd) => Ok(pwd),
            Err(_) => Err(anyhow!("Unable to get current directory")),
        }
    }

    fn get_storage_path() -> Result<PathBuf> {
        match env::var("HOME") {
            Ok(dir) => {
                let mut dir = PathBuf::from(dir);
                dir.push(".config");
                dir.push("projector.json");
                Ok(dir)
            }
            Err(_) => Err(anyhow!("Unable to get home directory")),
        }
    }
}
