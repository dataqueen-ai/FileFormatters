use serde_json::Value;
use std::fs::File;
use std::io::{self, Read, Write, BufWriter, Error};

fn main() -> Result<(), Error> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let data: Vec<Value> = serde_json::from_str(&buffer)?;

    let file = File::create("output.ndjson")?;
    let mut writer = BufWriter::new(file);

    for item in data {
        let ndjson = serde_json::to_string(&item)?;
        writeln!(writer, "{}", ndjson)?;
    }

    Ok(())
}
