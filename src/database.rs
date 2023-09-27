pub mod table;

use super::VaultError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use table::Table;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Database {
    pub(crate) name: String,
    tables: HashMap<String, Table>,
}

impl Database {
    pub(crate) fn new(name: String) -> Self {
        Self {
            name,
            tables: HashMap::new(),
        }
    }

    pub fn check_table(&self, name: &str) -> bool {
        self.tables.contains_key(name)
    }

    pub fn get_table(&self, name: &str) -> Result<&Table, VaultError> {
        self.tables.get(name).ok_or(VaultError::TableNotFound)
    }

    pub fn get_mut_table(&mut self, name: &str) -> Result<&mut Table, VaultError> {
        self.tables.get_mut(name).ok_or(VaultError::TableNotFound)
    }

    pub fn get_len(&self) -> usize {
        self.tables.len()
    }

    pub fn new_table(&mut self, name: &str) -> Result<(), VaultError> {
        if self.check_table(&name) {
            return Err(VaultError::TableNameTaken);
        }

        self.tables
            .insert(name.to_string(), Table::new(name.to_string()));
        Ok(())
    }

    pub fn rename_table(&mut self, name: &str, new_name: &str) -> Result<(), VaultError> {
        if self.check_table(new_name) {
            return Err(VaultError::TableNameTaken);
        }

        if let Some(mut table) = self.tables.remove(name) {
            table.name = new_name.to_string();
            self.tables.insert(new_name.to_string(), table);
            Ok(())
        } else {
            Err(VaultError::TableNotFound)
        }
    }

    pub fn delete_table(&mut self, name: &str) -> Result<(), VaultError> {
        if let Some(_table) = self.tables.remove(name) {
            Ok(())
        } else {
            Err(VaultError::TableNotFound)
        }
    }

    pub fn duplicate_table(&mut self, name: &str, new_name: &str) -> Result<(), VaultError> {
        if self.check_table(new_name) {
            return Err(VaultError::TableNameTaken);
        }

        if let Some(table) = self.tables.get(name) {
            let mut table = table.clone();
            table.name = new_name.to_string();
            self.tables.insert(new_name.to_string(), table);
            Ok(())
        } else {
            Err(VaultError::TableNotFound)
        }
    }
}
