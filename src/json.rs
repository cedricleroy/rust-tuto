
extern crate serde_json;

use std::fs::File;
use std::io::Read;
use serde_json::Value;


fn main() {
    let mut file = File::open("sample.json").expect("file not found");
    let mut content = String::new();
    file.read_to_string(&mut content);
    let data: Value = serde_json::from_str(&content).unwrap();
    println!("{}", data);
}

