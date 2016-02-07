mod binary_parser;

fn main() {
    let mut m: [u8; 32] = [0; 32];
    let _ = binary_parser::load_binary_to_memory("/tmp/test", &mut m);
    // let v = binary_parser::parse("/tmp/test");
    println!("Hello, world! {:?}", m);
}
