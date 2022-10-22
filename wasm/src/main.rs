#![no_main]

use flate2::{read::ZlibDecoder, write::ZlibEncoder, Compression};
use jpeg_encoder::{ColorType, Encoder};
use lopdf::{Document, Object, Stream};
use std::io::prelude::*;

enum FileType {
    Zlib,
    Jpeg,
}

/// Extended methods for lopdf::Stream used in this crate
trait StreamExtend {
    fn is_filetype(&self, file_type: FileType) -> bool;
    fn decompress_ex(&mut self);
    fn compress_ex(&mut self);
}

impl StreamExtend for Stream {
    /// File type determined by the file signature
    /// Further reading: https://en.wikipedia.org/wiki/List_of_file_signatures
    fn is_filetype(&self, file_type: FileType) -> bool {
        match file_type {
            FileType::Zlib => self.content.starts_with(&[0x78, 0x01]),
            FileType::Jpeg => self.content.starts_with(&[0xFF, 0xD8, 0xFF]),
        }
    }

    /// Decompress an extra layer of zlib-compressed files
    fn decompress_ex(&mut self) {
        let input = self.content.as_slice();
        let mut output = Vec::with_capacity(input.len() * 2);
        let mut decoder = ZlibDecoder::new(input);

        if !input.is_empty() {
            #[allow(clippy::unnecessary_lazy_evaluations, unused_variables)]
            decoder.read_to_end(&mut output).unwrap_or_else(|err| {
                // println!("{}", err);
                0
            });
        }

        self.set_content(output)
    }

    /// An extra step of zlib compression without marking `allows_compression` on `Stream`
    /// This follows some PDFs that have an extra zlib compression on their images
    fn compress_ex(&mut self) {
        let input = self.content.as_slice();
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::best());
        encoder.write_all(input).unwrap();

        self.set_content(encoder.finish().unwrap())
    }
}

/// Take a PDF binary and output compressed PDF binary
/// May panic on unexpected behavior
/// image_quality must be in range of 1-100
pub fn compress_pdf(from: &[u8], image_quality: u8) {
    let mut doc = Document::load_mem(from).unwrap();

    for object in doc.objects.values_mut() {
        if let Object::Stream(ref mut stream) = *object {
            // Images may have an extra layer
            let mut is_image_zlib = false;

            stream.decompress();

            // This decompress an extra layer of compressed JPEG image
            if stream.is_filetype(FileType::Zlib) {
                stream.decompress_ex();
                is_image_zlib = true;
            }

            if stream.is_filetype(FileType::Jpeg) {
                let mut buf = Vec::<u8>::new();
                let encoder = Encoder::new(&mut buf, image_quality);

                if let Ok(image) = image::load_from_memory(&stream.content) {
                    match encoder.encode(
                        image.as_bytes(),
                        image.width() as u16,
                        image.height() as u16,
                        match image.color() {
                            image::ColorType::L8 => ColorType::Luma,
                            image::ColorType::La8 => ColorType::Luma,
                            image::ColorType::Rgb8 => ColorType::Rgb,
                            image::ColorType::Rgba8 => ColorType::Rgba,
                            // TODO: handle other color types
                            _ => panic!("Not supported"),
                        },
                    ) {
                        Ok(_) => stream.set_content(buf),
                        Err(e) => {
                            println!("Error {e}");
                        }
                    }

                    if is_image_zlib {
                        stream.compress_ex();
                    }
                } else {
                    let _ = stream.compress();
                }
            }

            if stream.allows_compression {
                // Ignore any error and continue to compress other streams.
                let _ = stream.compress();
            }
        }
    }

    // TODO: export to bytes
    // doc.save()
}
