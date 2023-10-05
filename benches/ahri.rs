use ahri::Table;
use criterion::{criterion_group, criterion_main, Criterion};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Book {
    id: u32,
    title: String,
    author: String,
    year: u16,
    isbn: String,
    price: u16,
}

#[derive(Serialize, Deserialize, Debug)]
struct Customer {
    id: u32,
    first_name: String,
    last_name: String,
    email: String,
    phone: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Order {
    id: u32,
    customer_id: u32,
    date: String,
    total: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct OrderDetail {
    id: u32,
    order_id: u32,
    book_id: u32,
    quantity: u32,
    subtotal: u32,
}

const OPERATION_COUNT: u32 = 1000;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Ahri Operations", |b| {
        b.iter(|| {
            // SECTION BOOKS
            let book_structure = Book {
                id: 0,
                title: "Undefined".to_string(),
                author: "Undefined".to_string(),
                isbn: "Undefined".to_string(),
                price: 0,
                year: 0,
            };

            let mut books = Table::from_structure(book_structure);

            for i in 1..OPERATION_COUNT {
                let title = format!("Harry Potter {}", i + 1);
                books.insert(
                    &title.clone(),
                    Book {
                        id: i,
                        title,
                        author: "J. K. Rowling".to_string(),
                        isbn: "978-0-7475-3269-6".to_string(),
                        price: 7,
                        year: 1997 + i as u16,
                    },
                );
            }

            // SECTION CUSTOMERS
            let customer_structure = Customer {
                id: 0,
                first_name: "Undefined".to_string(),
                last_name: "Undefined".to_string(),
                email: "Undefined".to_string(),
                phone: "Undefined".to_string(),
            };

            let mut customers = Table::from_structure(customer_structure);

            for i in 1..OPERATION_COUNT {
                let first_name = format!("John {}", i + 1);
                let last_name = format!("Doe {}", i + 1);
                let email = format!("{}@example.com", i + 1);
                let phone = format!("+1-541-754-3010");
                customers.insert(
                    &first_name.clone(),
                    Customer {
                        id: i,
                        first_name,
                        last_name,
                        email,
                        phone,
                    },
                );
            }

            // SECTION ORDERS
            let order_structure = Order {
                id: 0,
                customer_id: 0,
                date: "Undefined".to_string(),
                total: 0,
            };

            let mut orders = Table::from_structure(order_structure);

            for i in 1..OPERATION_COUNT {
                orders.insert(
                    &i.to_string().clone(),
                    Order {
                        id: i,
                        customer_id: i,
                        date: "2020-12-12".to_string(),
                        total: 7,
                    },
                );
            }

            // SECTION ORDER DETAILS
            let order_detail_structure = OrderDetail {
                id: 0,
                order_id: 0,
                book_id: 0,
                quantity: 0,
                subtotal: 0,
            };

            let mut order_details = Table::from_structure(order_detail_structure);

            for i in 1..OPERATION_COUNT {
                order_details.insert(
                    &i.to_string().clone(),
                    OrderDetail {
                        id: i,
                        order_id: i,
                        book_id: i,
                        quantity: 1,
                        subtotal: 7,
                    },
                );
            }
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
