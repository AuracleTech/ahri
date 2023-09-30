use ahri::Table;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Book {
    name: String,
    year: u16,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let default_book = Book {
        name: "Unknown Book".to_string(),
        year: 0,
    };

    let mut bookshelf = Table::from_structure(default_book);

    for i in 0..7 {
        let book_name = format!("Harry Potter {}", i + 1);
        let new_book = Book {
            name: book_name.clone(),
            year: 1997 + i as u16,
        };
        bookshelf.insert(&book_name, new_book);
    }

    println!("Bookshelf: {:#?}", bookshelf);

    Ok(())
}
