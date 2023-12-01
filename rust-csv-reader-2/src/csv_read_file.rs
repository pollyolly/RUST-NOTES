use std::error::Error;
use std::io;

use csv;

pub fn read_using_file() -> Result<(), Box<dyn Error>> {
    println!("You may now read the csv using file path.");
    
    let mut path = String::new();
    io::stdin().read_line(&mut path).expect("Invalid file path");
    
    let mut reader = csv::Reader::from_path(&path)?;

    let headers = reader.headers()?;

    println!("{:?}", headers);

    for result in reader.records(){
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}
