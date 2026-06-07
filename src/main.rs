use rsraw::RawImage;

fn main() {
    let f = std::fs::read("raws/test.RAF").unwrap();
    let raw = RawImage::open(&f).unwrap();
    let info = raw.full_info();
    println!("Camera: {} {}", info.make, info.model);
}
