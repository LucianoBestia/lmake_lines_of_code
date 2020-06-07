//! count_lines_mod.rs

use crate::utilsmod::*;

#[allow(unused_imports)]
use ansi_term::Colour::{Green, Yellow};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{env, fs, path::Path};
use unwrap::unwrap;

#[derive(Clone, Debug, Default)]
pub struct LinesOfCode {
    /// lines with code in srs files
    pub src_code_lines: usize,
    /// lines with doc_comments in srs files
    pub src_doc_comment_lines: usize,
    /// lines with comments in srs files
    pub src_comment_lines: usize,
    /// unit plus integration tests
    pub tests_lines: usize,
    /// all lines in examples files
    pub examples_lines: usize,
}

fn one_project_count_lines(project_path: &Path) -> LinesOfCode {
    let mut lines_of_code = LinesOfCode::default();

    // src folder
    let files = unwrap!(traverse_dir_with_exclude_dir(
        &project_path.join("src"),
        "/*.rs",
        // avoid big folders and other folders with *.crev
        &vec![
            "/.git".to_string(),
            "/target".to_string(),
            "/docs".to_string()
        ]
    ));
    // println!("{:#?}", files);
    for rs_file_name in files.iter() {
        //dbg!(&rs_file_name);
        // Open the file in read-only mode (ignoring errors).
        let file = File::open(rs_file_name).unwrap();
        let reader = BufReader::new(file);
        let mut is_unit_test = false;
        // Read the file line by line using the lines() iterator from std::io::BufRead.
        for line in reader.lines() {
            let line = line.unwrap(); // Ignore errors.
            let line = line.trim_start();
            if line.starts_with("///") || line.starts_with("//!") {
                lines_of_code.src_doc_comment_lines += 1;
            } else if line.starts_with("//") || line.starts_with("/!") {
                lines_of_code.src_comment_lines += 1;
            } else if line.starts_with("#[cfg(test)]") {
                is_unit_test = true;
            } else if is_unit_test == true {
                lines_of_code.tests_lines += 1;
            } else {
                lines_of_code.src_code_lines += 1;
            }
        }
    }
    // tests folder
    let files = unwrap!(traverse_dir_with_exclude_dir(
        &project_path.join("tests"),
        "/*.rs",
        // avoid big folders and other folders with *.crev
        &vec![
            "/.git".to_string(),
            "/target".to_string(),
            "/docs".to_string()
        ]
    ));
    // println!("{:#?}", files);
    for rs_file_name in files.iter() {
        //dbg!(&rs_file_name);
        // Open the file in read-only mode (ignoring errors).
        let file = File::open(rs_file_name).unwrap();
        let reader = BufReader::new(file);
        // Read the file line by line using the lines() iterator from std::io::BufRead.
        for _line in reader.lines() {
            lines_of_code.tests_lines += 1;
        }
    }

    // examples folder
    let files = unwrap!(traverse_dir_with_exclude_dir(
        &project_path.join("examples"),
        "/*.rs",
        // avoid big folders and other folders with *.crev
        &vec![
            "/.git".to_string(),
            "/target".to_string(),
            "/docs".to_string()
        ]
    ));
    for rs_file_name in files.iter() {
        //dbg!(&rs_file_name);
        // Open the file in read-only mode (ignoring errors).
        let file = File::open(rs_file_name).unwrap();
        let reader = BufReader::new(file);
        // Read the file line by line using the lines() iterator from std::io::BufRead.
        for _line in reader.lines().enumerate() {
            lines_of_code.examples_lines += 1;
        }
    }
    //println!("{:#?}", &lines_of_code);
    // return
    lines_of_code
}

use serde_derive::Deserialize;

#[derive(Deserialize)]
struct CargoToml {
    workspace: Option<Workspace>,
}

#[derive(Deserialize)]
struct Workspace {
    members: Vec<String>,
}

pub fn workspace_or_project_count_lines() -> LinesOfCode {
    let mut lines_of_code = LinesOfCode::default();

    let current_dir = unwrap!(env::current_dir());
    println!(
        "current_dir: {}",
        Yellow.paint(unwrap!(current_dir.to_str()))
    );

    // cargo toml contains the list of projects
    let cargo_toml = unwrap!(fs::read_to_string("Cargo.toml"));
    let cargo_toml: CargoToml = unwrap!(toml::from_str(&cargo_toml));
    if let Some(workspace) = cargo_toml.workspace {
        for member in workspace.members.iter() {
            println!("{}", &member);
            let v = one_project_count_lines(&current_dir.join(member));
            lines_of_code.src_code_lines += v.src_code_lines;
            lines_of_code.src_doc_comment_lines += v.src_doc_comment_lines;
            lines_of_code.src_comment_lines += v.src_comment_lines;
            lines_of_code.tests_lines += v.tests_lines;
            lines_of_code.examples_lines += v.examples_lines;
        }
    } else {
        lines_of_code = one_project_count_lines(&current_dir);
    }
    // return
    lines_of_code
}

/// markdown can have nice tables
pub fn as_md_table(lines_of_code:LinesOfCode)->String{
    format!("
| src code | doc comments | comments | examples | tests |
| :------: | :----------: | :------: | :------: | :---: |
| {:^8   } | {:^12      } | {:^8   } | {:^8   } | {:^5} |",
lines_of_code.src_code_lines,
lines_of_code.src_doc_comment_lines,
lines_of_code.src_comment_lines,
lines_of_code.examples_lines,
lines_of_code.tests_lines)
}