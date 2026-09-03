use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read};

fn main() {
    check_all_boxes_in_mp4().unwrap();

    let vec = vec![0, 1, 176, 219];

    println!("{:?}", convert_first_four_bytes(&vec));
}

struct BoxHeader {
    size: u32,
    name: str,
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

        println!("{:?}", &buff[..32]);
    }

    Ok(())
}


fn convert_first_four_bytes(box_size: &Vec<u8>) -> u32 {
    let mut result_size = 0u32;

    for byte in box_size {
        result_size = result_size * 256 + *byte as u32;
    }
    result_size
}

fn convert_bytes_to_ascii(box_size: &Vec<u8>, start_pointer: usize, end_pointer: usize) -> String {

    let temp_box: Vec<u8> = box_size[start_pointer..end_pointer].to_vec();

    let result_name = String::from_utf8(temp_box).unwrap();

    result_name

}