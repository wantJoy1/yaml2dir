extern crate yaml_rust;

use yaml_rust::{YamlLoader};
use std::env;
use std::error;
use std::fs;

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<_> = env::args().collect();
    let ref fname_in = args[1];

    let content = fs::read_to_string(fname_in);
    let docs = YamlLoader::load_from_str(&content.unwrap()).unwrap();
    let doc = &docs[0];
    println!("doc: Array\n{:?}", doc);

    /*
    for element in doc {

    }
    */

    Ok(())
}

fn do_mkdir(dirname: String) {
    fs::create_dir(dirname);
}