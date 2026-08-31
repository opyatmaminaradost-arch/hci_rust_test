use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

fn main() -> io::Result<()>{

    if let Some(file) = find_config_values("file.link")? {
        let metadata = file.metadata()?;

        println!("Size: {}", metadata.len());
        println!("Is file: {}", metadata.is_file());
    }

    Ok(())

}

fn find_config_values(pattern: &str) -> io::Result<Option<File>> {

    let config_lines = File::open("config.properties").unwrap();
    let reader = BufReader::new(config_lines);

    for line in reader.lines() {
        let line = line?;

        if let Some((key,value)) = line.split_once('='){
            if key.trim() == pattern {
                let path = value
                        .trim()
                        .trim_matches('"');

                let file = File::open(path)?;

                return Ok(Some(file));
            }
        }
    }
    Ok(None)
}