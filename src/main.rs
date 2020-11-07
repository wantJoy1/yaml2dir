extern crate yaml_rust;
extern crate xml;

use yaml_rust::{YamlLoader};
use std::env;
use std::error;
use std::fs;

fn main() -> Result<(), Box<dyn error::Error>> {

    do_mkdir("jape/unchi");

    let args: Vec<_> = env::args().collect();

    if args.len() == 1 {
        panic!("invalid argument.");
    }

    for i in 1..args.len() {
        let ref fname_in = args[i];
        let content = fs::read_to_string(fname_in);
        let docs = YamlLoader::load_from_str(&content.unwrap()).unwrap();
        let doc = &docs[0];
        println!("doc\n");
        println!("{:?}", doc);

        // mkdir_recursive(doc);
    }

    Ok(())
}

// fn mkdir_recursive(dirs: Array[String])

fn do_mkdir(file_name: &str) -> u8 {
    match fs::create_dir_all(file_name) {
        Err(e) => panic!("{}: {}", file_name, e),
        Ok(_) => 0,
    }
}