use std::{env, fs};

use rsraw::{BIT_DEPTH_16, RawImage};
use tiff::encoder::{TiffEncoder, colortype};

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <file>", args[0]);
        std::process::exit(2);
    }
    let file = &args[1];
    let data = fs::read(file).unwrap();
    let mut image = RawImage::open(&data).unwrap();

    let info = image.full_info();

    println!("Camera: {} {}", info.make, info.model);
    image.unpack().unwrap();

    let processed = image.process::<BIT_DEPTH_16>().unwrap();

    let out = fs::File::create("target/out.tiff").unwrap();
    let mut tiff = TiffEncoder::new(out).unwrap();

    tiff.write_image::<colortype::RGB16>(info.height, info.width, &processed)
        .unwrap();
}
