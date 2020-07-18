use glob::glob;
use rayon::prelude::*;
use colored::*;
use regex::Regex;
use std::path::{Path, PathBuf};
use std::fs;
use std::fmt;

/// When a path is found
pub struct PathFinded {
    /// The path to where the file is
    filepath: String,
    /// The line number on the path
    line_number: u64,
    /// The path that the regex found
    path: String,
}

impl fmt::Display for PathFinded{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[ {} ] File {:<20} line {:>04} --> {:<20}", "x".red().bold(), self.filepath, self.line_number, self.path)
    }
}

/// Check if a codebase has any absolute paths
///
/// # Arguments
///
/// * `path` - The location of the codebase
///
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

/// Check if this entry has any absolute paths, if this is a folder, no nothing
/// 
/// # Arguments
///
/// * `path` - Location of the file
/// * `set` - Vector of Regex that will be use to search for absolute paths
///
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

/// Check if the string has any absolute paths
/// 
/// # Arguments
///
/// * `lines` - Content of a file
/// * `set` - Vector of Regex that will be use to search for absolute paths
/// * `file` - Path to the initial file (useful to build PathFinded instances)
///
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

/// Check if this file has any absolute paths
/// 
/// # Arguments
///
/// * `path` - Location of the file
/// * `set` - Vector of Regex that will be use to search for absolute paths
///
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