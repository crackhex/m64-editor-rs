#![feature(ascii_char)]
#![feature(int_roundings)]

mod m64_handling;
mod file_handling;

use std::path::Path;
use anyhow::Result;
use crate::m64_handling::{M64File};
use crate::file_handling::{read_file, save_file};


pub fn main() -> Result<()>{
    let input_path = "Path\\To\\Input.m64";
    let m64_input_path = Path::new(input_path);
    let output_path  ="Path\\To\\Output.m64";
    let m64_output_path = Path::new(output_path);
    let mut m64 = M64File::from_bytes(&read_file(m64_input_path)?)?;
    match save_file(m64_output_path, &m64.to_bytes()?){
        Ok(_) => println!("File saved successfully"),
        Err(e) => println!("Failed to save file: {}", e)
    }
    Ok(())
}