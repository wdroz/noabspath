#[macro_use]
extern crate clap;
use colored::*;

mod checkabspath;

use checkabspath::check_codebase;

fn main() {
    let matches = clap_app!(noabspath =>
        (version: "0.1.4")
        (author: "William Droz <william.droz.ch@gmail.com>")
        (about: "check that there aren't absolute paths in codebases")
        (@arg PATH: +required "path of codebase to check")
    ).get_matches();
    let folder = matches.value_of("PATH").unwrap();
    println!("check absolute path in : {}", folder);
    let res = check_codebase(folder.to_string());
    match res {
        Ok(_) => {
            println!("[ {} ] not absolute paths", "OK".green().bold());
            std::process::exit(0)
        },
        Err(paths) => {
            for path in paths {
                println!("{}", path);
            }
            std::process::exit(-1)
        }
    }
}