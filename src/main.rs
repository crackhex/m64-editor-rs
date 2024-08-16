#![feature(ascii_char)]
#![feature(int_roundings)]
extern crate core;

mod m64_handling;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use crate::m64_handling::{M64File};

fn open_file(path: &Path) -> io::Result<File> {
    Ok(File::open(path)?)
}

pub fn main() -> io::Result<()>{
    let input_path = "Path\\To\\Input.m64";
    let m64_input_path = Path::new(input_path);
    let output_path  ="Path\\To\\Output.m64";
    let m64_output_path = Path::new(output_path);
    let m64 = M64File::build_m64(&mut open_file(m64_input_path)?).unwrap();
    m64.save_m64(m64_output_path).expect("TODO: panic message");
    Ok(())

}