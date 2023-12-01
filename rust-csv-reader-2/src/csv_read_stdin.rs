use std::error::Error;
use std::io;

use csv;

pub fn read_using_stdin() -> Result<(), Box<dyn Error>> {
    println!("You may now read the file using stdin.");
    //create new csv Reader
    let mut reader = csv::Reader::from_reader(io::stdin());
    let headers = reader.headers()?;

    println!("Headers: {:?}", headers);

    for result in reader.records(){ 
         let record = result?;

         println!("{:?}", record);
    }
    Ok(())
}
