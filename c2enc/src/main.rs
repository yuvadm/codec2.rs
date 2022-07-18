extern crate byteorder;
extern crate libcodec2;

use byteorder::{LittleEndian, ReadBytesExt};
use std::fs::File;
use std::io::prelude::*;
use std::io::Cursor;
use std::path::Path;

fn main() {
    let path = Path::new("../../codec2/wav/cross.wav");
    let mut file = File::open(&path).unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();
    let mut rdr = Cursor::new(buf);
    while let Ok(res) = rdr.read_i16::<LittleEndian>() {
        println!("{}", res);
    }
}
