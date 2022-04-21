use clap::Parser;
use colored::*;

mod checkabspath;

use checkabspath::check_codebase;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path of codebase to check
    #[clap(short, long)]
    path: String,

    /// File that contains patterns to ignore
    #[clap(short, long, default_value = ".gitignore")]
    ignore_file: String,
}


fn main() {
    let args = Args::parse();
    let folder = args.path;
    let ignore_file = args.ignore_file;
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
