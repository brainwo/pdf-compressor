use flate2::{read::ZlibDecoder, write::ZlibEncoder, Compression};
use std::io::prelude::*;

pub(crate) fn decompress_extra(input: &[u8]) -> Vec<u8> {
    let mut output = Vec::with_capacity(input.len() * 2);
    let mut decoder = ZlibDecoder::new(input);

    if !input.is_empty() {
        decoder.read_to_end(&mut output).unwrap_or_else(|err| {
            // println!("{}", err);
            0
        });
    }

    output
}

pub(crate) fn compress_extra(input: &[u8]) -> Vec<u8> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::best());
    encoder.write_all(input).unwrap();

    encoder.finish().unwrap()
}
