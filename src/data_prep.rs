use csv::ReaderBuilder;
use std::io::Read;
use crate::Highway;
pub fn load_and_filter_highways<R: Read>(reader: R) -> Result<Vec<Highway>, csv::Error> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(reader);
    let mut highways: Vec<Highway> = Vec::new();

    for result in rdr.deserialize::<Highway>() {
        match result {
            Ok(highway) if highway.removed.is_none() => {
                highways.push(highway);
            },
            Ok(_) => {},
            Err(e) => {
                eprintln!("Error parsing CSV record: {:?}", e);
                continue;
            }
        }
    }

    Ok(highways)
}
