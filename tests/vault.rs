use ahri::Vault;

const FOLDER: &str = "./prototyping/";
const SAVE_PATH: &str = "./prototyping/test.ahri";

const DATABASE_NAME: &str = "league of legends";

#[test]
fn create_vault() {
    Vault::new();
}

#[test]
fn serialize_deserialize() {
    std::fs::create_dir_all(FOLDER).unwrap();

    let mut ahri = Vault::new();

    ahri.serialize(SAVE_PATH).unwrap();
    let serialized_file_size = std::fs::metadata(SAVE_PATH).unwrap().len();

    ahri.new_database(DATABASE_NAME).unwrap();

    ahri.serialize(SAVE_PATH).unwrap();
    let new_serialized_file_size = std::fs::metadata(SAVE_PATH).unwrap().len();
    assert_ne!(serialized_file_size, new_serialized_file_size);

    match Vault::deserialize(SAVE_PATH) {
        Ok(vault_deserialized) => assert!(vault_deserialized.check_database(DATABASE_NAME)),
        Err(err) => panic!("Error: {}", err),
    }

    std::fs::remove_dir_all(FOLDER).unwrap();
}
