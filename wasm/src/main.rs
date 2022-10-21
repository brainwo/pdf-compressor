#![no_main]

mod helper;

use helper::{compress_extra, decompress_extra};
use jpeg_encoder::{ColorType, Encoder};
use lopdf::{Document, Object};

/// image_quality must be in range of 1-100
pub fn compress_pdf(from: &[u8], image_quality: u8) {
    let mut doc = Document::load_mem(from).unwrap();

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

    // TODO: export to bytes
    // doc.save()
}
