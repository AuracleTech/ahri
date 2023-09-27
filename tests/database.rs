use ahri::Vault;

const DATABASE_NAME: &str = "league of legends";
const DATABASE_RENAMED: &str = "riot games";

#[test]
fn create_database() {
    let mut vault = Vault::new();
    vault.new_database(DATABASE_NAME).unwrap();
    assert!(vault.check_database(DATABASE_NAME));
}

#[test]
#[should_panic(expected = "DatabaseNameTaken")]
fn create_existing_database() {
    let mut vault = Vault::new();
    vault.new_database(DATABASE_NAME).unwrap();
    vault.new_database(DATABASE_NAME).unwrap();
}

#[test]
#[should_panic(expected = "DatabaseNotFound")]
fn get_invalid_database() {
    let mut vault = Vault::new();
    vault.new_database(DATABASE_NAME).unwrap();
    vault.get_database(DATABASE_RENAMED).unwrap();
}

#[test]
fn rename_database() {
    let mut vault = Vault::new();
    vault.new_database(DATABASE_NAME).unwrap();
    vault
        .rename_database(DATABASE_NAME, DATABASE_RENAMED)
        .unwrap();
    assert!(!vault.check_database(DATABASE_NAME));
    assert!(vault.check_database(DATABASE_RENAMED));
}

#[test]
#[should_panic(expected = "DatabaseNameTaken")]
fn rename_database_to_taken_name() {
    let mut vault = Vault::new();
    vault.new_database(DATABASE_NAME).unwrap();
    vault
        .rename_database(DATABASE_RENAMED, DATABASE_NAME)
        .unwrap();
}

#[test]
#[should_panic(expected = "DatabaseNotFound")]
fn rename_invalid_database() {
    let mut vault = Vault::new();
    vault
        .rename_database(DATABASE_NAME, DATABASE_RENAMED)
        .unwrap();
}

#[test]
fn duplicate_database() {
    let mut vault = Vault::new();
    vault.new_database(DATABASE_NAME).unwrap();
    vault
        .duplicate_database(DATABASE_NAME, DATABASE_RENAMED)
        .unwrap();
    assert!(vault.check_database(DATABASE_NAME));
    assert!(vault.check_database(DATABASE_RENAMED));
}

#[test]
#[should_panic(expected = "DatabaseNameTaken")]
fn duplicate_database_to_taken_name() {
    let mut vault = Vault::new();
    vault.new_database(DATABASE_NAME).unwrap();
    vault
        .duplicate_database(DATABASE_RENAMED, DATABASE_NAME)
        .unwrap();
}

#[test]
#[should_panic(expected = "DatabaseNotFound")]
fn duplicate_unknown_database() {
    let mut vault = Vault::new();
    vault
        .duplicate_database(DATABASE_NAME, DATABASE_RENAMED)
        .unwrap();
}
