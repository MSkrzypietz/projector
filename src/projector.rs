use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    env,
    fs::{self, File, OpenOptions},
    io::Write,
    path::PathBuf,
};

use crate::config::{Config, Operation};

#[derive(Debug, Serialize)]
pub struct Projector {
    #[serde(skip_serializing)]
    config: Config,
    data: HashMap<PathBuf, HashMap<String, String>>,
}

impl Projector {
    pub fn from_config(config: Config) -> Projector {
        Projector {
            config,
            data: HashMap::new(),
        }
    }

    pub fn execute(&mut self) -> Result<(), &'static str> {
        match &self.config.operation {
            Operation::List => {
                self.add(String::from("hello"), String::from("world"));
                self.add(String::from("hello"), String::from("bar"));

                let res = self.list();
                println!("{:?}", res);
                ()
            }
            Operation::Add(key, value) => {
                self.add(key.clone(), value.clone());
                self.save();
            }
            Operation::Remove(key) => self.remove(key.clone()),
        }

        Ok(())
    }

    fn list(&self) -> HashMap<&String, &String> {
        let mut dirs = Vec::new();
        let mut curr = Some(self.config.pwd.as_path());
        while let Some(p) = curr {
            dirs.push(p);
            curr = p.parent();
        }

        let mut out = HashMap::new();
        for dir in dirs.into_iter().rev() {
            if let Some(map) = self.data.get(dir) {
                out.extend(map);
            }
        }
        out
    }

    fn add(&mut self, key: String, value: String) {
        self.data
            .entry(self.config.pwd.clone())
            .or_default()
            .insert(key, value);
    }

    fn remove(&mut self, key: String) {
        self.data
            .entry(self.config.pwd.clone())
            .or_default()
            .remove(&key);
    }

    fn save(&self) {
        let serialized = serde_json::to_string(&self.data).unwrap();

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(get_data_path())
            .unwrap();

        file.write_all(serialized.as_bytes()).unwrap();
        file.flush().unwrap();
    }
}

fn get_data_path() -> PathBuf {
    let dir = env::var("HOME").unwrap();
    let mut dir = PathBuf::from(dir);

    dir.push(".config");
    dir.push("projector.json");
    dir
}
