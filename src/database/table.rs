use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
enum TableValue {
    Boolean(bool),
    Char(char),
    U8(u8),
    I8(i8),
    U32(u32),
    I32(i32),
    F32(f32),
    I64(i64),
    F64(f64),
    String(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Column {
    name: String,
    values: Vec<TableValue>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Table {
    pub(crate) name: String,
    uuid_column: Column,
    columns: Vec<Column>,
}

impl Table {
    pub(crate) fn new(name: String) -> Self {
        Self {
            name,
            uuid_column: Column {
                name: String::new(),
                values: Vec::new(),
            },
            columns: Vec::new(),
        }
    }
}
