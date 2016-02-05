#![feature(step_by)]
mod binary_parser;

fn main() {
    let v = binary_parser::parse("/tmp/test");
    println!("Hello, world! {:?}", v);
}
