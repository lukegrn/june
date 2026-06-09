use std::{env, fs::File};

use tiff::encoder::{colortype, TiffEncoder};

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <file>", args[0]);
        std::process::exit(2);
    }
    let file = &args[1];
    let image = rawloader::decode_file(file).unwrap();
    let (width, height) = (image.width, image.height);

    println!("Model: {}", image.model);

    let file = File::create("target/out.tiff").unwrap();
    let mut tiff = TiffEncoder::new(file).unwrap();
    let mut img_bytes = Vec::<u8>::new();

    if let rawloader::RawImageData::Integer(data) = image.data {
        for pixel in data {
            let pixh = (pixel >> 8) as u8;
            // let pixl = (pixel & 0x0ff) as u8;
            img_bytes.push(pixh);
            img_bytes.push(pixh);
            img_bytes.push(pixh);
        }
    } else {
        eprintln!("Don't know how to process non-integer raw files");
    }

    tiff.write_image::<colortype::RGB8>(
        width.try_into().unwrap(),
        height.try_into().unwrap(),
        &img_bytes,
    )
    .unwrap();
}
