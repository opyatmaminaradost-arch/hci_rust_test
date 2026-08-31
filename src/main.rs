use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read};

fn main() {
    check_all_boxes_in_mp4().unwrap();
}

fn find_config_values(pattern: &str) -> io::Result<Option<File>> {

    let config_lines = File::open("config.properties")?;
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


fn check_all_boxes_in_mp4() -> io::Result<()> {

    if let Some(mut file) = find_config_values("file.link")? {

        let mut buff = Vec::new();

        file.read_to_end(&mut buff)?;

        println!("Readed: {}", buff.len());
    }

    Ok(())
}