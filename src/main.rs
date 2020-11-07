/*
fn main() {
    println!("Hello, world!");
}
*/

extern crate yaml_rust;
use yaml_rust::{YamlLoader};

// https://github.com/chyh1990/yaml-rust の写経
fn main() {
    let s = 
        "
    servers:
        - a.example.com
        - b.example.com
        - c.example.com
        - d.example.com
    ";
    let docs = YamlLoader::load_from_str(s).unwrap();
   let doc = &docs[0]; 

   println!("{:?}", doc);
}