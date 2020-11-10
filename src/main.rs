extern crate yaml_rust;
use yaml_rust::{Yaml, YamlLoader};
use std::env;
use std::error;
use std::fs;

fn main() -> Result<(), Box<dyn error::Error>> {

    // 引数を取得して、 args に格納
    let args: Vec<String> = env::args().collect();

    // 引数がないときにエラーを出して強制終了
    if args.len() == 1 {
        panic!("too few arguments");
    }

    // 1つめの引数のパスで指されたファイルの中の文字列を content に格納,
    // 失敗したらエラー投げて終了
    let content = match fs::read_to_string(&args[1]) {
        Ok(s)  => s,
        Err(_) => panic!("file reading error")
    };
    // YamlLoader に文字列 content を渡してYamlとして解析させる
    // 失敗したらエラー投げて終了
    let docs = match YamlLoader::load_from_str(&content) {
        Ok(d)  => d,
        Err(_) => panic!("parse error")
    };
    // Vec<Yaml> 型の配列 docs のそれぞれの要素を doc に格納する
    for doc in docs {
        // doc の中身を出力
        println!("------doc------");
        println!("{:?}", doc);
        /*
         * println!("------inspect------");
         * inspect(&doc, 0);
         */
        // doc を scan_mkdir に渡す
        println!("------scan_mkdir------");
        scan_mkdir(&doc, 0, ".".to_string());
    }

    Ok(())
}


// for develop
fn inspect(y: &Yaml, n: usize) -> () {
    match y {
        Yaml::Hash(d) => {
            for (k, v) in d {
                if let Yaml::String(s) = k {
                    print!("{}", "  ".repeat(n));
                    println!("{}", s);
                }
                inspect(v, n + 1)
            }
        }
        _ => { println!("not implemented yet.") }
    }
}

// y から下の構造に向けてディレクトリを掘っていく関数
// n はネストの深さ（出力のため）
fn scan_mkdir(y: &Yaml, n: usize, path: String) -> () {
    match y {
        Yaml::Hash(d) => {
            for (k, v) in d {
                if let Yaml::String(s) = k {
                    print!("{}", "  ".repeat(n));
                    println!("{}", s);
                    // 新たなパスを表す文字列を作り、それを再帰的に scan_mkdir へ渡す
                    scan_mkdir(v, n+1, format!("{}/{}",path.to_string(),s));
                }                
            }
        },
        _ => {
            println!("mkdir -p {}", path);
            do_mkdir(&path);
        }
    }
}

// 文字列からディレクトリを作る
fn do_mkdir(file_name: &str) -> u8 {
    match fs::create_dir_all(file_name) {
        Err(e) => panic!("{}: {}", file_name, e),
        Ok(_) => 0
    }
}