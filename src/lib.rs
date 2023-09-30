use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error, fs::File, io::Write};

#[derive(Serialize, Deserialize, Debug)]
pub struct Table<T> {
    default: T,
    rows: HashMap<String, T>,
}

impl<T> Table<T> {
    pub fn from_structure(default: T) -> Self {
        Self {
            default,
            rows: HashMap::new(),
        }
    }

    pub fn from_file(path: &str) -> Result<Self, Box<dyn Error>>
    where
        T: for<'de> Deserialize<'de>,
    {
        let file = File::open(path)?;
        let deserialized_data = bincode::deserialize_from(file)?;
        Ok(deserialized_data)
    }

    pub fn to_file(&self, path: &str) -> Result<(), Box<dyn Error>>
    where
        T: Serialize,
    {
        let mut file = File::create(path)?;
        let serialized_data = bincode::serialize(self)?;
        file.write_all(&serialized_data)?;
        Ok(())
    }

    pub fn insert(&mut self, key: &str, row: T) {
        self.rows.insert(key.to_string(), row);
    }

    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    pub fn get(&self, key: &str) -> Option<&T> {
        self.rows.get(key)
    }
}
