use criterion::{criterion_group, criterion_main, Criterion};
use mysql::{params, prelude::Queryable, Pool};
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

const MYSQL: &str = "mysql://root:butter@localhost:3306/random";

const OPERATION_COUNT: u32 = 1000;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("MySQL Operations", |b| {
        b.iter(|| {
            let pool = Pool::new(MYSQL).unwrap();
            let mut conn = pool.get_conn().unwrap();

            // SECTION BOOKS
            conn.query_drop(
                r"CREATE TEMPORARY TABLE books (
                id INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
                title TEXT NOT NULL,
                author TEXT NOT NULL,
                isbn TEXT NOT NULL,
                price INT NOT NULL,
                year INT NOT NULL
            )",
            )
            .unwrap();

            for i in 1..OPERATION_COUNT {
                let title = format!("Harry Potter {}", i + 1);
                conn.exec_drop(
                    r"INSERT INTO books (id, title, author, isbn, price, year)
                    VALUES (:id, :title, :author, :isbn, :price, :year)",
                    params! {
                        "id" => i,
                        "title" => title,
                        "author" => "J. K. Rowling",
                        "isbn" => "978-0-7475-3269-6",
                        "price" => 7,
                        "year" => 1997 + i as u16,
                    },
                )
                .unwrap();
            }

            // SECTION CUSTOMERS
            conn.query_drop(
                r"CREATE TEMPORARY TABLE customers (
                id INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
                first_name TEXT NOT NULL,
                last_name TEXT NOT NULL,
                email TEXT NOT NULL,
                phone TEXT NOT NULL
            )",
            )
            .unwrap();

            for i in 1..OPERATION_COUNT {
                let first_name = format!("John {}", i + 1);
                let last_name = format!("Doe {}", i + 1);
                let email = format!("{}@example.com", i + 1);
                let phone = format!("+1-541-754-3010");
                conn.exec_drop(
                    r"INSERT INTO customers (id, first_name, last_name, email, phone)
                    VALUES (:id, :first_name, :last_name, :email, :phone)",
                    params! {
                        "id" => i,
                        "first_name" => first_name,
                        "last_name" => last_name,
                        "email" => email,
                        "phone" => phone,
                    },
                )
                .unwrap();
            }

            // SECTION ORDERS
            conn.query_drop(
                r"CREATE TEMPORARY TABLE orders (
                id INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
                customer_id INT NOT NULL,
                date TEXT NOT NULL,
                total INT NOT NULL
            )",
            )
            .unwrap();

            for i in 1..OPERATION_COUNT {
                conn.exec_drop(
                    r"INSERT INTO orders (id, customer_id, date, total)
                    VALUES (:id, :customer_id, :date, :total)",
                    params! {
                        "id" => i,
                        "customer_id" => i,
                        "date" => "2020-12-12".to_string(),
                        "total" => 7,
                    },
                )
                .unwrap();
            }

            // SECTION ORDER DETAILS
            conn.query_drop(
                r"CREATE TEMPORARY TABLE order_details (
                id INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
                order_id INT NOT NULL,
                book_id INT NOT NULL,
                quantity INT NOT NULL,
                subtotal INT NOT NULL
            )",
            )
            .unwrap();

            for i in 1..OPERATION_COUNT {
                conn.exec_drop(
                    r"INSERT INTO order_details (id, order_id, book_id, quantity, subtotal)
                    VALUES (:id, :order_id, :book_id, :quantity, :subtotal)",
                    params! {
                        "id" => i,
                        "order_id" => i,
                        "book_id" => i,
                        "quantity" => 1,
                        "subtotal" => 0,
                    },
                )
                .unwrap();
            }
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
