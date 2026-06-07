use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <file>", args[0]);
        std::process::exit(2);
    }
    let file = &args[1];
    let image = rawloader::decode_file(file).unwrap();

    println!("Model: {}", image.model);
}
