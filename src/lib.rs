mod database;

use database::Database;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error, fmt::Display, fs::File, io::Write};

#[derive(Debug)]
pub enum VaultError {
    DatabaseNameTaken,
    DatabaseNotFound,

    TableNameTaken,
    TableNotFound,

    SerializationError(bincode::Error),
    IoError(std::io::Error),
}

impl Error for VaultError {}
impl Display for VaultError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VaultError::DatabaseNameTaken => write!(f, "Database name taken"),
            VaultError::DatabaseNotFound => write!(f, "Database not found"),

            VaultError::TableNameTaken => write!(f, "Table name taken"),
            VaultError::TableNotFound => write!(f, "Table not found"),

            VaultError::SerializationError(e) => write!(f, "Serialization error: {}", e),
            VaultError::IoError(e) => write!(f, "IO error: {}", e),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Vault {
    databases: HashMap<String, Database>,
}

impl Vault {
    pub fn new() -> Self {
        Self {
            databases: HashMap::new(),
        }
    }

    pub fn deserialize(path: &str) -> Result<Self, VaultError> {
        let file = File::open(path).map_err(VaultError::IoError)?;
        let deserialized_data =
            bincode::deserialize_from(file).map_err(VaultError::SerializationError)?;
        Ok(deserialized_data)
    }

    pub fn serialize(&self, path: &str) -> Result<(), Box<dyn Error>> {
        let mut file = File::create(path).map_err(VaultError::IoError)?;
        let serialized_data = bincode::serialize(self).map_err(VaultError::SerializationError)?;
        file.write_all(&serialized_data)
            .map_err(VaultError::IoError)?;
        Ok(())
    }

    pub fn contains_database(&self, name: &str) -> bool {
        self.databases.contains_key(name)
    }

    pub fn get_database(&self, name: &str) -> Result<&Database, VaultError> {
        self.databases.get(name).ok_or(VaultError::DatabaseNotFound)
    }

    pub fn get_mut_database(&mut self, name: &str) -> Result<&mut Database, VaultError> {
        self.databases
            .get_mut(name)
            .ok_or(VaultError::DatabaseNotFound)
    }

    pub fn database_count(&self) -> usize {
        self.databases.len()
    }

    pub fn new_database(&mut self, name: &str) -> Result<(), VaultError> {
        if self.contains_database(name) {
            return Err(VaultError::DatabaseNameTaken);
        }

        self.databases
            .insert(name.to_string(), Database::new(name.to_string()));

        Ok(())
    }

    pub fn rename_database(&mut self, name: &str, new_name: &str) -> Result<(), VaultError> {
        if self.contains_database(new_name) {
            return Err(VaultError::DatabaseNameTaken);
        }

        if let Some(mut database) = self.databases.remove(name) {
            database.name = new_name.to_string();
            self.databases.insert(new_name.to_string(), database);
            Ok(())
        } else {
            Err(VaultError::DatabaseNotFound)
        }
    }

    pub fn delete_database(&mut self, name: &str) -> Result<(), VaultError> {
        if !self.contains_database(name) {
            return Err(VaultError::DatabaseNotFound);
        }

        self.databases.remove(name);

        Ok(())
    }

    pub fn duplicate_database(&mut self, name: &str, new_name: &str) -> Result<(), VaultError> {
        if self.contains_database(new_name) {
            return Err(VaultError::DatabaseNameTaken);
        }

        if let Some(database) = self.databases.get(name) {
            let mut cloned_database = database.clone();
            cloned_database.name = new_name.to_string();
            self.databases
                .insert(new_name.to_string(), cloned_database.clone());
            Ok(())
        } else {
            Err(VaultError::DatabaseNotFound)
        }
    }
}
