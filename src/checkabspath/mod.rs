use colored::*;
use glob::glob;
use rayon::prelude::*;
use regex::Regex;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use wildmatch::WildMatch;
/// Simple trait to detect if the path exist from a string. Return empty string if no match
pub trait PathDetection {
    fn path_exist(&self, line: &str) -> String;
}

/// When a path is found
pub struct PathFinded {
    /// The path to where the file is
    filepath: String,
    /// The line number on the path
    line_number: u64,
    /// The path that the regex found
    path: String,
}

impl fmt::Display for PathFinded {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[ {} ] File {:<20} line {:>04} --> {:<20}",
            "x".red().bold(),
            self.filepath,
            self.line_number,
            self.path
        )
    }
}
/// Specific regex for paths
pub struct RegExForPath {
    regex: Regex,
}

/// Use internal regex to check for a path
impl PathDetection for RegExForPath {
    fn path_exist(&self, line: &str) -> String {
        let mut res: String = String::from("");
        for caps in self.regex.captures_iter(line) {
            if let Some(cap) = caps.iter().next() {
                res = cap.unwrap().as_str().to_string();
                return res;
            }
        }
        res
    }
}
/// Set of specific regex for paths
pub struct RegExSetForPath {
    regex_set: Vec<RegExForPath>,
}

/// Use all specific regex in a loop to search for paths
impl PathDetection for RegExSetForPath {
    fn path_exist(&self, line: &str) -> String {
        let mut res: String = String::from("");
        for regex in &self.regex_set[..] {
            res = regex.path_exist(line);
            if !res.is_empty() {
                return res;
            }
        }
        res
    }
}

/// Read .gitignore style file and return a Vec of WildMatch.
/// If the there is an error when reading the file, return None
///
fn create_wildmatches_from_file(filename: String) -> Option<Vec<WildMatch>> {
    let contents = fs::read_to_string(filename.clone());
    match contents {
        Ok(lines) => {
            let res: Vec<WildMatch> = lines
                .lines()
                .par_bridge()
                .map(|x| format!("*{}*", x))
                .map(|x| WildMatch::new(&x))
                .collect();
            Some(res)
        }
        Err(_) => {
            eprintln!("Warning: Unable to read {}. ignoring", filename.clone());
            None
        }
    }
}

/// Returns true if there is a match, false otherwise
///
fn is_ignored(path: &Path, wildmatches: &Vec<WildMatch>) -> bool {
    for wildmatch in wildmatches {
        if wildmatch.matches(path.to_str().unwrap()) {
            return true;
        }
    }
    false
}

/// Create default set of regexes to search for absolute paths
///
fn create_regexes_for_abs_paths() -> RegExSetForPath {
    RegExSetForPath {
        regex_set: vec![
            // Unix absolute paths: e.g. "/usr/local/bin"
            RegExForPath {
                regex: Regex::new(r#"[\"']\/(?:[^\/'"]+\/)*[^\/'"]+[\"']"#).unwrap(),
            },
            // Windows absolute paths with drive letter: e.g. "C:\Program Files\App"
            RegExForPath {
                regex: Regex::new(r#"[\"'][A-Za-z]:[\\/][^'"]+[\"']"#).unwrap(),
            },
            // Windows UNC paths: e.g. "\\server\share\folder"
            RegExForPath {
                regex: Regex::new(r#"[\"'](?:\\\\|//)[^\\/]+[\\/][^'"]+[\"']"#).unwrap(),
            },
        ],
    }
}

/// Check if a codebase has any absolute paths
///
/// # Arguments
///
/// * `path` - The location of the codebase
///
pub fn check_codebase(path: String, ignore_file: String) -> Result<(), Vec<PathFinded>> {
    let set = create_regexes_for_abs_paths();
    let mut glob_expression = path;
    if glob_expression.ends_with("/") {
        glob_expression.push_str("**/*");
    } else {
        glob_expression.push_str("/**/*");
    }
    let potential_files: Vec<PathBuf> = glob(&glob_expression)
        .unwrap()
        .map(|x| x.unwrap())
        .collect();
    let wildmatches = create_wildmatches_from_file(ignore_file);
    let potential_files_filtered = match wildmatches {
        Some(matches) => potential_files
            .into_iter()
            .par_bridge()
            .filter(|x| !is_ignored(x, &matches))
            .collect(),
        None => potential_files,
    };
    let entries: Vec<Vec<PathFinded>> = potential_files_filtered
        .par_iter()
        .map(move |entry| check_entry(entry, &set))
        .flatten()
        .collect();

    if entries.is_empty() {
        Ok(())
    } else {
        Err(entries.into_iter().flatten().collect())
    }
}

/// Check if this entry has any absolute paths, if this is a folder, no nothing
///
/// # Arguments
///
/// * `path` - Location of the file
/// * `set` - Implementation of PathDetection that will be use to search for absolute paths
///
fn check_entry(path: &Path, set: &impl PathDetection) -> Option<Vec<PathFinded>> {
    let metadata = path.metadata();
    match metadata {
        Ok(x) => {
            if x.is_file() {
                check_one_file(path, set)
            } else {
                None
            }
        }
        Err(_) => {
            eprintln!(
                "Warning: Unable to read metadata of {}. ignoring",
                path.to_str().unwrap()
            );
            None
        }
    }
}

/// Check if the string has any absolute paths
///
/// # Arguments
///
/// * `lines` - Content of a file
/// * `set` - Implementation of PathDetection that will be use to search for absolute paths
/// * `file` - Path to the initial file (useful to build PathFinded instances)
///
fn fill_from_content(lines: &str, set: &impl PathDetection, file: &Path) -> Vec<PathFinded> {
    let mut res = std::vec::Vec::new();
    for (nb, line) in lines.lines().enumerate() {
        let path = set.path_exist(line);
        if !path.is_empty() {
            if let Some(filepath) = file.to_str() {
                res.push(PathFinded {
                    filepath: filepath.to_string(),
                    line_number: 1 + nb as u64,
                    path,
                });
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
/// * `set` - Implementation of PathDetection that will be use to search for absolute paths
///
fn check_one_file(file: &Path, set: &impl PathDetection) -> Option<Vec<PathFinded>> {
    let contents = fs::read_to_string(file);
    match contents {
        Ok(lines) => {
            let res = fill_from_content(&lines, set, file);
            match res.is_empty() {
                true => None,
                false => Some(res),
            }
        }
        Err(_) => None,
    }
}

// tests

#[cfg(test)]
mod checkabspath {
    use super::*;

    #[test]
    fn test_check_one_file_regex_find() {
        let regex = RegExForPath {
            regex: Regex::new(r"William Droz").unwrap(),
        };
        let my_file = Path::new("src/checkabspath/mod.rs");
        let res = check_one_file(&my_file, &regex);
        assert!(res.is_some())
    }
    #[test]
    fn test_check_one_file_regex_not_find() {
        let regex = RegExForPath {
            regex: Regex::new(r"William Droz").unwrap(),
        };
        let my_file = Path::new(".gitignore");
        let res = check_one_file(&my_file, &regex);
        assert!(res.is_none())
    }
}
