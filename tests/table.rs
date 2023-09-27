use ahri::Vault;

const DATABASE_NAME: &str = "league of legends";

const TABLE_NAME: &str = "players";
const TABLE_RENAMED: &str = "users";

#[test]
fn create_table() {
    let mut vault = Vault::new();
    vault.new_database(DATABASE_NAME).unwrap();
    let database = vault.get_mut_database(DATABASE_NAME).unwrap();
    database.new_table(TABLE_NAME).unwrap();
    assert!(database.contains_table(TABLE_NAME));
}

#[test]
#[should_panic(expected = "TableNameTaken")]
fn create_existing_table() {
    let mut vault = Vault::new();
    vault.new_database(DATABASE_NAME).unwrap();
    let database = vault.get_mut_database(DATABASE_NAME).unwrap();
    database.new_table(TABLE_NAME).unwrap();
    database.new_table(TABLE_NAME).unwrap();
}

#[test]
#[should_panic(expected = "TableNotFound")]
fn get_invalid_table() {
    let mut vault = Vault::new();
    vault.new_database(DATABASE_NAME).unwrap();
    let database = vault.get_mut_database(DATABASE_NAME).unwrap();
    database.new_table(TABLE_NAME).unwrap();
    database.get_table(TABLE_RENAMED).unwrap();
}

#[test]
fn rename_table() {
    let mut vault = Vault::new();
    vault.new_database(DATABASE_NAME).unwrap();
    let database = vault.get_mut_database(DATABASE_NAME).unwrap();
    database.new_table(TABLE_NAME).unwrap();
    database.rename_table(TABLE_NAME, TABLE_RENAMED).unwrap();
    assert!(!database.contains_table(TABLE_NAME));
    assert!(database.contains_table(TABLE_RENAMED));
}

#[test]
#[should_panic(expected = "TableNameTaken")]
fn rename_table_to_taken_name() {
    let mut vault = Vault::new();
    vault.new_database(DATABASE_NAME).unwrap();
    let database = vault.get_mut_database(DATABASE_NAME).unwrap();
    database.new_table(TABLE_NAME).unwrap();
    database.new_table(TABLE_RENAMED).unwrap();
    database.rename_table(TABLE_RENAMED, TABLE_NAME).unwrap();
}

#[test]
#[should_panic(expected = "TableNotFound")]
fn rename_invalid_table() {
    let mut vault = Vault::new();
    vault.new_database(DATABASE_NAME).unwrap();
    let database = vault.get_mut_database(DATABASE_NAME).unwrap();
    database.rename_table(TABLE_NAME, TABLE_RENAMED).unwrap();
}

#[test]
fn duplicate_table() {
    let mut vault = Vault::new();
    vault.new_database(DATABASE_NAME).unwrap();
    let database = vault.get_mut_database(DATABASE_NAME).unwrap();
    database.new_table(TABLE_NAME).unwrap();
    database.duplicate_table(TABLE_NAME, TABLE_RENAMED).unwrap();
    assert!(database.contains_table(TABLE_NAME));
    assert!(database.contains_table(TABLE_RENAMED));
}

#[test]
#[should_panic(expected = "TableNameTaken")]
fn duplicate_table_to_taken_name() {
    let mut vault = Vault::new();
    vault.new_database(DATABASE_NAME).unwrap();
    let database = vault.get_mut_database(DATABASE_NAME).unwrap();
    database.new_table(TABLE_NAME).unwrap();
    database.new_table(TABLE_RENAMED).unwrap();
    database.duplicate_table(TABLE_NAME, TABLE_RENAMED).unwrap();
}

#[test]
#[should_panic(expected = "TableNotFound")]
fn duplicate_unknown_table() {
    let mut vault = Vault::new();
    vault.new_database(DATABASE_NAME).unwrap();
    let database = vault.get_mut_database(DATABASE_NAME).unwrap();
    database.duplicate_table(TABLE_NAME, TABLE_RENAMED).unwrap();
}
