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
    let mut speech = Vec::new();
    while let Ok(i) = rdr.read_i16::<LittleEndian>() {
        speech.push(i);
    }

    let mut allbits = Vec::new();
    let c = libcodec2::Codec2::new();
    let nsam = c.samples_per_frame();
    let _nbits = c.bytes_per_frame();

    for speech in speech.chunks(nsam) {
        let mut bits: [u8; 7] = [0,0,0,0,0,0,0]; // nbits assume 7
        c.encode(&speech, &mut bits);
        allbits.extend(bits);
    }

    println!("{:?}", allbits);
}
