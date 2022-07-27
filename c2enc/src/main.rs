extern crate byteorder;
extern crate libcodec2;

use byteorder::{LittleEndian, ReadBytesExt};
use std::fs::File;
use std::io::prelude::*;
use std::io::Cursor;
use std::path::Path;

struct Cli {
    in_path: std::path::PathBuf,
    out_path: std::path::PathBuf,
}

fn main() {
    let in_path_arg = std::env::args().nth(1).expect("no input path given");
    let out_path_arg = std::env::args().nth(2).expect("no output path given");

    let args = Cli {
        in_path: std::path::PathBuf::from(in_path_arg),
        out_path: std::path::PathBuf::from(out_path_arg),
    };

    let in_path = Path::new(&args.in_path);
    let mut file = File::open(&in_path).unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();

    let mut rdr = Cursor::new(buf);
    let mut speech = Vec::new();
    while let Ok(i) = rdr.read_i16::<LittleEndian>() {
        speech.push(i);
    }

    let c = libcodec2::Codec2::new();
    let nsam = c.samples_per_frame();
    let nbits = c.bytes_per_frame();

    let mut file = File::create(&args.out_path).unwrap();

    for samples in speech.chunks_exact(nsam) {
        // remainder frame will be dropped
        let mut bits = vec![0u8; nbits];
        c.encode(&samples, &mut bits);
        file.write(&bits).unwrap();
    }
    file.flush().unwrap();
}
