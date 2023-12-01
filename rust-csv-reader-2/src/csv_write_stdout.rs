use std::error::Error;
use csv;
use std::io;

pub fn write_to_stdout() -> Result<(), Box<dyn Error>> {

    let mut writer = csv::Writer::from_writer(io::stdout());

    writer.write_record(&[
        "Customer_guid",
        "first_name",
        "last_name",
        "email",
        "address"
    ])?;
    writer.write_record(&[
        "8d748d7f-82fs9d8fdsf-8s8d8f9d9f023",
        "Ailey",
        "Benten",
        "smaple@gmail.com",
        "456 San Miguel",
    ])?;
    writer.write_record(&[
        "8s9d8s0d948-8sddf8d9d-6s723h3jf8",
        "Dionisia",
        "Guilbert",
        "kurimaw@gmail.com",
        "748 Mandawa Eskinita"
    ])?;

    writer.flush()?;

    Ok(())
}
