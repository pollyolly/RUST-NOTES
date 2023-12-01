use serde::Deserialize;
use std::io;
use std::error::Error;
use csv;

#[derive(Debug, Deserialize)]
struct Customer {
    customer_guid: String,
    first_name: String,
    last_name: String,
    email: String, 
    address: String,
}

pub fn read_serde_file() -> Result <(), Box<dyn Error>> {
    println!("You may now input the file path.");
    let mut path = String::new();

    io::stdin().read_line(&mut path).expect("Invalid Input");

    let mut reader = csv::Reader::from_path(&path)?;

    let headers = reader.headers();

    println!("{:?}", headers);

    for result in reader.deserialize() {
        let record: Customer =  result?;

        println!("{:?}", record);
    }
    Ok(())
}
