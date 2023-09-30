use ahri::Table;
use serde::{Deserialize, Serialize};

const FOLDER: &str = "./prototyping/";
const FILE_PATH: &str = "./prototyping/test.ahri";

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: u8,
}

#[test]
fn serialize_deserialize() {
    std::fs::create_dir_all(FOLDER).unwrap();

    let structure = User {
        name: "Undefined".to_string(),
        age: 0,
    };

    let mut table = Table::from_structure(structure);
    assert!(table.row_count() == 0);
    table.to_file(FILE_PATH).unwrap();
    let serialized_file_size = std::fs::metadata(FILE_PATH).unwrap().len();

    table.insert(
        "bob",
        User {
            name: "Bob".to_string(),
            age: 30,
        },
    );
    table.insert(
        "alice",
        User {
            name: "Alice".to_string(),
            age: 20,
        },
    );
    table.to_file(FILE_PATH).unwrap();
    let new_serialized_file_size = std::fs::metadata(FILE_PATH).unwrap().len();

    assert!(new_serialized_file_size > serialized_file_size);

    match Table::<User>::from_file(FILE_PATH) {
        Ok(table) => assert!(table.row_count() == 2),
        Err(err) => panic!("{}", err),
    }

    std::fs::remove_dir_all(FOLDER).unwrap();
}

#[test]
fn create_table() {
    let structure = User {
        name: "Undefined".to_string(),
        age: 0,
    };

    let mut table = Table::from_structure(structure);
    assert!(table.row_count() == 0);

    table.insert(
        "bob",
        User {
            name: "Bob".to_string(),
            age: 30,
        },
    );
    table.insert(
        "alice",
        User {
            name: "Alice".to_string(),
            age: 20,
        },
    );
    assert!(table.row_count() == 2);
}
