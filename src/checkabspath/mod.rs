use glob::glob;
use rayon::prelude::*;
use regex::Regex;
use std::path::{Path, PathBuf};
use std::fs;


#[derive(Debug)]
pub struct PathFinded {
    filepath: String,
    line_number: u64,
    path: String,
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

fn check_one_file(file: &Path, set: &Vec<Regex>) -> Option<Vec<PathFinded>> {
    let contents = fs::read_to_string(file);
    match contents {
        Ok(lines) => {
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