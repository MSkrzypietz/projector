use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    env,
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    projector: HashMap<PathBuf, HashMap<String, String>>,
}

#[derive(Debug)]
pub struct Projector {
    pwd: PathBuf,
    storage: PathBuf,
    data: Data,
}

impl Projector {
    pub fn from_config(pwd: PathBuf) -> Projector {
        let storage = get_storage_path();
        let data = if Path::new(&storage).exists() {
            let raw_data = fs::read_to_string(&storage).unwrap();
            serde_json::from_str(&raw_data).unwrap()
        } else {
            Data {
                projector: HashMap::new(),
            }
        };

        Projector { pwd, storage, data }
    }

    pub fn get_all_values(&self) -> HashMap<&String, &String> {
        let mut dirs = Vec::new();
        let mut curr = Some(self.pwd.as_path());
        while let Some(p) = curr {
            dirs.push(p);
            curr = p.parent();
        }

        let mut out = HashMap::new();
        for dir in dirs.into_iter().rev() {
            if let Some(map) = self.data.projector.get(dir) {
                out.extend(map);
            }
        }
        out
    }

    pub fn add(&mut self, key: String, value: String) {
        self.data
            .projector
            .entry(self.pwd.clone())
            .or_default()
            .insert(key, value);
    }

    pub fn remove(&mut self, key: String) {
        self.data
            .projector
            .entry(self.pwd.clone())
            .or_default()
            .remove(&key);
    }

    pub fn save(&self) {
        let serialized = serde_json::to_string(&self.data).unwrap();

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&self.storage)
            .unwrap();

        file.write_all(serialized.as_bytes()).unwrap();
        file.flush().unwrap();
    }
}

fn get_storage_path() -> PathBuf {
    let dir = env::var("HOME").unwrap();
    let mut dir = PathBuf::from(dir);

    dir.push(".config");
    dir.push("projector.json");
    dir
}
