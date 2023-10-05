# ahri

ahri is a database 🗳

##### Completed

- [x] Database structure
- [ ] Async support
- [x] Serialization / Deserialization
- [ ] ACID transactions

##### Example

```rust
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
```

##### Result

```rust,ignore
Bookshelf: Table {
    default: Book {
        name: "Unknown Book",
        year: 0,
    },
    rows: {
        "Harry Potter 3": Book {
            name: "Harry Potter 3",
            year: 1999,
        },
        "Harry Potter 6": Book {
            name: "Harry Potter 6",
            year: 2002,
        },
        "Harry Potter 7": Book {
            name: "Harry Potter 7",
            year: 2003,
        },
        "Harry Potter 2": Book {
            name: "Harry Potter 2",
            year: 1998,
        },
        "Harry Potter 4": Book {
            name: "Harry Potter 4",
            year: 2000,
        },
        "Harry Potter 5": Book {
            name: "Harry Potter 5",
            year: 2001,
        },
        "Harry Potter 1": Book {
            name: "Harry Potter 1",
            year: 1997,
        },
    },
}
```

##### Performances

creation of 4 tables and for each inserting 1000 entries in `766 µs`
