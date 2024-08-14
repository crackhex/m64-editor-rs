#![feature(ascii_char)]
extern crate core;

mod m64_handling;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use crate::m64_handling::{M64File};

fn open_file(path: &str) -> io::Result<File> {
    let f = File::open(path)?;
    Ok(f)
}

pub fn main() -> io::Result<()>{
    let m64_path = "PATH\\TO\\M64";
    let m64_name = "gogg";
    let m64_file_path = format!("{}\\{}.m64", m64_path, m64_name);
    let mut file = open_file(&m64_file_path)?;
    let m64 = M64File::build_m64(&mut file);
    match m64 {
        Ok(m64) => {
            println!("{:?}", m64.internal_name);
        },
        Err(e) => {
            println!("{:?}", e);
        }
    }
   // let internal_name = &m64.header.internal_name;
   // println!("{:?}", internal_name);
    Ok(())
}