extern crate byteorder;
extern crate libcodec2;

use byteorder::{LittleEndian, ReadBytesExt};
use std::fs::File;
use std::io::prelude::*;
use std::io::Cursor;
use std::path::Path;

use libcodec2::{Codec2, Modes};

struct Cli {
    bit_rate: String,
    in_path: std::path::PathBuf,
    out_path: std::path::PathBuf,
}

const USAGE: &str = "Usage: c2enc <BITRATE> <INPUT_FILE> <OUTPUT_FILE>";

fn main() {
    let bitrate_arg = std::env::args().nth(1).expect(USAGE);
    let in_path_arg = std::env::args().nth(2).expect(USAGE);
    let out_path_arg = std::env::args().nth(3).expect(USAGE);

    let args = Cli {
        bit_rate: String::from(bitrate_arg),
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

    let mode = match args.bit_rate.as_str() {
        "3200" => Modes::Mode3200,
        "2400" => Modes::Mode2400,
        "1600" => Modes::Mode1600,
        "1400" => Modes::Mode1400,
        "1300" => Modes::Mode1300,
        "1200" => Modes::Mode1200,
        "700C" => Modes::Mode700C,
        "450" => Modes::Mode450,
        _ => Modes::Mode1400,
    };

    let c = Codec2::new(mode);
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
