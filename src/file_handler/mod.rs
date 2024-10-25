use std::fs::File;
use std::io::{self, Write, Read};



pub fn write_to_file(content: &str, path: &str) -> io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())
}

pub fn read_from_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    let size = file.read_to_string(&mut content)?;
    println!("size : {}", size);
    Ok(content)
}

