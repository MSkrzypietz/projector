use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{self, OpenOptions},
    io::Write,
    path::PathBuf,
};

#[derive(Debug, Default, Serialize, Deserialize)]
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
    pub fn from_config(pwd: PathBuf, storage: PathBuf) -> Projector {
        let data = match fs::read_to_string(&storage) {
            Ok(raw_data) => serde_json::from_str(&raw_data).unwrap_or_default(),
            Err(_) => Data::default(),
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

    pub fn get_value(&self, key: &str) -> Option<&String> {
        let mut curr = Some(self.pwd.as_path());
        while let Some(p) = curr {
            if let Some(map) = self.data.projector.get(p) {
                if let Some(val) = map.get(key) {
                    return Some(val);
                }
            }
            curr = p.parent();
        }

        None
    }

    pub fn add(&mut self, key: String, value: String) {
        self.data
            .projector
            .entry(self.pwd.clone())
            .or_default()
            .insert(key, value);
    }

    pub fn remove(&mut self, key: &str) {
        self.data
            .projector
            .get_mut(&self.pwd)
            .map(|entry| entry.remove(key));
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

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    macro_rules! hashmap {
        ($($k:expr => $v:expr),* $(,)?) => {{
            core::convert::From::from([$(($k, $v),)*])
        }};
    }

    fn mock_projector(pwd: PathBuf) -> Projector {
        let data = Data {
            projector: hashmap! {
                PathBuf::from("/foo/bar") => hashmap! {
                    String::from("a") => String::from("b"),
                    String::from("s") => String::from("t"),
                },
                PathBuf::from("/foo/bar/baz") => hashmap! {
                    String::from("a") => String::from("z"),
                    String::from("c") => String::from("d"),
                },
            },
        };

        Projector {
            pwd,
            storage: PathBuf::default(),
            data,
        }
    }

    #[test]
    fn get_all_values_for_root() {
        let projector = mock_projector(PathBuf::from("/foo/bar"));

        assert_eq!(
            projector.get_all_values(),
            hashmap! {
                &String::from("a") => &String::from("b"),
                &String::from("s") => &String::from("t"),
            }
        );
    }

    #[test]
    fn get_all_values_for_leaf() {
        let projector = mock_projector(PathBuf::from("/foo/bar/baz"));

        assert_eq!(
            projector.get_all_values(),
            hashmap! {
                &String::from("a") => &String::from("z"),
                &String::from("c") => &String::from("d"),
                &String::from("s") => &String::from("t"),
            }
        );
    }

    #[test]
    fn get_value_for_root() {
        let projector = mock_projector(PathBuf::from("/foo/bar"));

        assert_eq!(projector.get_value("a"), Some(&String::from("b")));
        assert_eq!(projector.get_value("s"), Some(&String::from("t")));
    }

    #[test]
    fn get_value_for_leaf() {
        let projector = mock_projector(PathBuf::from("/foo/bar/baz"));

        assert_eq!(projector.get_value("a"), Some(&String::from("z")));
        assert_eq!(projector.get_value("c"), Some(&String::from("d")));
        assert_eq!(projector.get_value("s"), Some(&String::from("t")));
    }

    #[test]
    fn add() {
        let mut projector = mock_projector(PathBuf::from("/foo/bar/baz"));
        projector.add(String::from("hello"), String::from("world"));

        assert_eq!(projector.get_value("hello"), Some(&String::from("world")));
    }

    #[test]
    fn remove() {
        let mut projector = mock_projector(PathBuf::from("/foo/bar/baz"));
        projector.add(String::from("hello"), String::from("world"));
        projector.remove("hello");

        assert_ne!(projector.get_value("hello"), Some(&String::from("world")));
    }
}
