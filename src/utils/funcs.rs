use std::{fs::File, io::{Read, self}};


pub fn get_file_content(file_path: String) -> Result<String, io::Error> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}