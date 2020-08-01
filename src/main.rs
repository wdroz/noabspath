#[macro_use]
extern crate clap;
use colored::*;

mod checkabspath;

use checkabspath::check_codebase;

fn main() {
    let matches = clap_app!(noabspath =>
        (version: "0.1.6")
        (author: "William Droz <william.droz.ch@gmail.com>")
        (about: "check that there aren't obvious absolute paths in codebases")
        (@arg PATH: +required "path of codebase to check")
        (@arg IGNORE_FILE: -i --ignore_file +takes_value "File that contains patterns to ignore (default .gitignore)")
    ).get_matches();
    let folder = matches.value_of("PATH").unwrap();
    let ignore_file = matches
        .value_of("IGNORE_FILE")
        .unwrap_or(".gitignore")
        .to_string();
    println!("check absolute path in : {}", folder);
    let res = check_codebase(folder.to_string(), ignore_file);
    match res {
        Ok(_) => {
            println!("[ {} ] not absolute paths", "OK".green().bold());
            std::process::exit(0)
        }
        Err(paths) => {
            for path in paths {
                println!("{}", path);
            }
            std::process::exit(-1)
        }
    }
}
