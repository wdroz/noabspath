use glob::glob;
use rayon::prelude::*;
use colored::*;
use regex::Regex;
use std::path::{Path, PathBuf};
use std::fs;
use std::fmt;

pub struct PathFinded {
    filepath: String,
    line_number: u64,
    path: String,
}

impl fmt::Display for PathFinded{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[ {} ] File {:<20} line {:>04} --> {:<20}", "x".red().bold(), self.filepath, self.line_number, self.path)
    }
}

pub fn check_codebase(path: String) -> Result<(), Vec<PathFinded>> {
    let set = vec![Regex::new("[\"']/\\w+/?[^'\"]+[\"']").unwrap(),
                   Regex::new("[\"']\\\\w+\\?[^'\"]+[\"']").unwrap(),];
    let mut glob_expression = path;
    if glob_expression.ends_with("/") {
        glob_expression.push_str("**/*");
    }
    else {
        glob_expression.push_str("/**/*");
    }
    let potential_files: Vec<PathBuf> = glob(&glob_expression).unwrap().into_iter().map(|x| x.unwrap()).collect();
    let entries: Vec<Vec<PathFinded>> = potential_files.par_iter().map(move |entry| check_entry(&entry, &set)).flatten().collect();
    
    if entries.is_empty() {
        return Ok(());
    }
    else {
        return Err(entries.into_iter().flatten().collect());
    }
}

fn check_entry(path: &Path, set: &Vec<Regex>) -> Option<Vec<PathFinded>>
{
    let metadata = path.metadata();
    match metadata {
        Ok(x) => {
            if x.is_file() {
                check_one_file(path, set)
            }
            else {
                None
            }
        }
        Err(_) => None,
    }
}

fn fill_from_content(lines: &String, set: &Vec<Regex>, file: &Path) -> Vec<PathFinded> {
    let mut res = std::vec::Vec::new();
    for (nb, line) in lines.lines().enumerate(){
        for regex in set {
            for caps in regex.captures_iter(&line) {
                for cap in caps.iter() {
                    res.push(PathFinded {
                        filepath: file.to_str().unwrap().to_string(), 
                        line_number: 1+nb as u64, 
                        path: cap.unwrap().as_str().to_string(),
                        });
                }
            }
        }
    }
    res
}

fn check_one_file(file: &Path, set: &Vec<Regex>) -> Option<Vec<PathFinded>> {
    let contents = fs::read_to_string(file);
    match contents {
        Ok(lines) => {
            let res = fill_from_content(&lines, set, file);
            match res.is_empty() {
                true => None,
                false => Some(res)
            }
        }
        Err(_) => {
            //println!("can't read {:?}", file);
            None
        }
    }
}

// tests

#[cfg(test)]
mod checkabspath {
    use super::*;

    #[test]
    fn test_check_one_file_regex_find() {
        let set = vec![Regex::new(r"William Droz").unwrap()];
        let my_file = Path::new("src/checkabspath/mod.rs");
        let res = check_one_file(&my_file, &set);
        assert!(res.is_some())
    }
    #[test]
    fn test_check_one_file_regex_not_find() {
        let set = vec![Regex::new(r"William Droz").unwrap()];
        let my_file = Path::new(".gitignore");
        let res = check_one_file(&my_file, &set);
        assert!(res.is_none())
    }
}