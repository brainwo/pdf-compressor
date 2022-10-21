use flate2::Compression;
use jpeg_encoder::{ColorType, Encoder};
use lopdf::{Document, Object};

fn decompress_extra(input: &[u8]) -> Vec<u8> {
    use flate2::read::ZlibDecoder;
    use std::io::prelude::*;

    let mut output = Vec::with_capacity(input.len() * 2);
    let mut decoder = ZlibDecoder::new(input);

    if !input.is_empty() {
        decoder.read_to_end(&mut output).unwrap_or_else(|err| {
            println!("{}", err);
            0
        });
    }

    output
}

fn compress_extra(input: &[u8]) -> Vec<u8> {
    use flate2::write::ZlibEncoder;
    use std::io::prelude::*;

    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::best());
    encoder.write_all(input).unwrap();

    encoder.finish().unwrap()
}

fn main() -> Result<(), lopdf::Error> {
    let mut doc = Document::load("example.pdf")?;

    for object in doc.objects.values_mut() {
        if let Object::Stream(ref mut stream) = *object {
            let mut is_image_zlib = false;

            stream.decompress();

            // This decompress an extra layer of compressed JPEG image
            if stream.content.starts_with(&[0x78, 0x01]) {
                stream.set_content(decompress_extra(&stream.content));
                is_image_zlib = true;
            }

            // JPEG format
            if stream.content.starts_with(&[0xFF, 0xD8, 0xFF]) {
                let mut buf = Vec::<u8>::new();
                let encoder = Encoder::new(&mut buf, 10);

                if let Ok(image) = image::load_from_memory(&stream.content) {
                    // TODO: set width and image to 32 bit
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
                        stream.set_content(compress_extra(&stream.content));
                    }
                } else {
                    let _ = stream.compress();
                }
            } else if stream.allows_compression {
                // Ignore any error and continue to compress other streams.
                let _ = stream.compress();
            }
        }
    }

    doc.save("out.pdf")?;
    Ok(())
}
