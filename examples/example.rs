// use auth_server::Database;
// use rand::Rng;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let database = Database::create("huh")?;

    // let mut db_server = Database::new();
    // db_server.create_table("MY test table 🦨");

    // println!("Filling database with random keys and values...");

    // let mut rng = rand::thread_rng();
    // let amount_of_keys = u16::MAX;
    // let mut last_ping = std::time::Instant::now();

    // for i in 0..amount_of_keys {
    //     let random_key: String = i.to_string();
    //     let random_value: String = (0..32)
    //         .map(|_| (0x20u8 + (rng.gen::<f32>() * 96.0) as u8) as char)
    //         .collect();
    //     db_server.create_entry("test".to_string(), random_key, random_value);

    //     if last_ping.elapsed().as_secs() >= 1 {
    //         println!("Generated {}/{} keys", i, amount_of_keys);
    //         last_ping = std::time::Instant::now(); // Get the current time
    //     }
    // }

    // println!("Done");

    // println!("Accessing random value...");
    // println!("Size of HashMap: {}", hashmap.len());
    // assert_eq!(hashmap.len(), amount_of_keys as usize); // Assert that the hashmap contains the amount of keys defined above

    // let start = std::time::Instant::now(); // Get the current time
    // let random_value_to_get = (amount_of_keys / 2).to_string(); // Get the value at the middle of the hashmap
    // println!("Value: {}", hashmap.get(&random_value_to_get).unwrap()); // Get the value at the middle of the hashmap
    // println!("Time taken to get value: {}ms", start.elapsed().as_millis()); // Get the time elapsed since start

    Ok(())
}
