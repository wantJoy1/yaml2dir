use std::env;
use std::error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn error::Error>> {
    eprintln!("START");
    let args: Vec<_> = env::args().collect();
    let ref fname_in = args[1];

    for result in BufReader::new(File::open(fname_in)?).lines() {
        let ll = result?;
        println!("{}", ll);
    }

    eprintln!("FINISH");
    Ok(())
}

/*
extern crate yaml_rust;
use yaml_rust::{YamlLoader};

use std::fs;
use std::io;
use std::env;

// https://github.com/chyh1990/yaml-rust の写経
fn main() -> Result<(), Box<std::error::Error>> {
    // args[1]
    let file = fs::read_to_string(args[1])?;
    println!("{}", file);
    Ok(())
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

fn do_mkdir(dirname: String) {
    fs::create_dir(dirname);
}
*/