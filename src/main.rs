#![feature(ascii_char)]
#![feature(int_roundings)]
extern crate core;

mod m64_handling;
mod file_handling;

use std::io;
use std::io::prelude::*;
use std::path::Path;
use crate::m64_handling::{M64File};
use crate::file_handling::{read_file, save_file};



pub fn main() -> io::Result<()>{
    let input_path = "Path\\to\\m64\\input.m64";
    let m64_input_path = Path::new(input_path);
    let output_path  ="Path\\to\\m64\\output.m64";
    let m64_output_path = Path::new(output_path);
    let m64 = M64File::from_bytes(&read_file(m64_input_path)?).expect("Failed to parse M64 file");
    match save_file(m64_output_path, &m64.to_bytes()) {
        Ok(_) => println!("File saved successfully"),
        Err(e) => println!("Failed to save file: {}", e)
    }
    Ok(())

}