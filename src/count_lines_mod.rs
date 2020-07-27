//! count_lines_mod.rs

use crate::utilsmod::*;

#[allow(unused_imports)]
use ansi_term::Colour::{Green, Yellow};
use regex::Regex;
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

/// private function. Use public workspace_or_project_count_lines().
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

/// Returns the struct LinesOfCode for 4 types of lines:
/// code, doc comments, comments, test and examples.
/// Automatically detects if this is a workspace or single rust project.
///
/// # Example
///
/// ```
/// use lmake_lines_of_code::*;
///
/// let v = workspace_or_project_count_lines();
/// dbg!(&v);
/// ```
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

/// Returns a string with the code for a markdown table with count of lines.
///
/// Some websites render a beautiful table, but others render ugly tables.
/// Use badges instead.
///
/// # Example
///
/// ```
/// use lmake_lines_of_code::*;
///
/// let v = workspace_or_project_count_lines();
/// let badges = as_md_table(&v);
///
/// println!("{}", badges);
/// ```
pub fn as_md_table(lines_of_code: &LinesOfCode) -> String {
    // I added an empty row to have the next row with different color from the header.
    format!(
        "
| src code | doc comments | comments | examples | tests |
| :------: | :----------: | :------: | :------: | :---: |
|  lines   |     lines    |   lines  |   lines  | lines |
| {:^8   } | {:^12      } | {:^8   } | {:^8   } | {:^5} |

",
        lines_of_code.src_code_lines,
        lines_of_code.src_doc_comment_lines,
        lines_of_code.src_comment_lines,
        lines_of_code.examples_lines,
        lines_of_code.tests_lines
    )
}
/// Returns a string with the markdown code for 4 shield badges.
///
/// Every badge has the link to the url given as first CLI argument
/// or automatically finds out the github git remote repository url.
///
/// # Example
///
/// ```
/// use lmake_lines_of_code::*;
///
/// let v = workspace_or_project_count_lines();
/// let badges = as_shield_badges(&v,"", false,false);
///
/// println!("{}", badges);
/// ```
pub fn as_shield_badges(
    lines_of_code: &LinesOfCode,
    link: &str,
    is_testing_no_git: bool,
    is_testing_git_dummy_result: bool,
) -> String {
    println!("as_shield_badges() start");
    let cmd = if is_testing_no_git {
        "produce-an-error"
    } else {
        "git"
    };
    // use the CLI argument or automatically find GitHub git remote repository
    let link = if link.is_empty() {
        // find the repo name
        // $ git remote -v
        // returns
        // origin  git@github.com:LucianoBestia/lmake_lines_of_code.git (fetch)
        if let Ok(output) = std::process::Command::new(cmd)
            .arg("remote")
            .arg("-v")
            .output()
        {
            let output = if is_testing_git_dummy_result {
                // for testing I need a predictable result
                "origin  git@github.com:LucianoBestia/lmake_lines_of_code.git (fetch)".to_string()
            } else {
                String::from_utf8_lossy(&output.stdout).to_string()
            };
            if output.is_empty() {
                String::new()
            } else {
                println!("{}", &output);
                // regex capture 3 groups: website, user_name and repo_name
                let reg = unwrap!(Regex::new(r#"@(.*?):(.*?)/(.*?).git"#));
                let cap = unwrap!(reg.captures(&output));
                format!("https://{}/{}/{}/", &cap[1], &cap[2], &cap[3])
            }
        } else {
            println!("git error");
            String::new()
        }
    } else {
        println!("link parameter {}", link);
        link.to_string()
    };

    let src_code_lines = format!(
        "[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-{}-green.svg)]({})",
        lines_of_code.src_code_lines, link
    );
    let src_doc_comment_lines = format!("[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-{}-blue.svg)]({})",lines_of_code.src_doc_comment_lines,link);
    let src_comment_lines = format!(
        "[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-{}-purple.svg)]({})",
        lines_of_code.src_comment_lines, link
    );
    let example_lines = format!(
        "[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-{}-yellow.svg)]({})",
        lines_of_code.examples_lines, link
    );
    let tests_lines = format!(
        "[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-{}-orange.svg)]({})",
        lines_of_code.tests_lines, link
    );
    //return
    format!(
        "{}\n{}\n{}\n{}\n{}\n",
        src_code_lines, src_doc_comment_lines, src_comment_lines, example_lines, tests_lines
    )
}

#[cfg(test)]
mod test {
    use super::*;
    fn create_struct_for_test() -> LinesOfCode {
        //return
        LinesOfCode {
            src_code_lines: 11,
            src_doc_comment_lines: 22,
            src_comment_lines: 33,
            tests_lines: 44,
            examples_lines: 55,
        }
    }
    #[test]
    fn test_01_badge_with_arg() {
        let v = create_struct_for_test();
        let badges = as_shield_badges(&v, "http://website", false, false);

        assert_eq!(badges,"[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-11-green.svg)](http://website)\n[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-22-blue.svg)](http://website)\n[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-33-purple.svg)](http://website)\n[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-55-yellow.svg)](http://website)\n[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-44-orange.svg)](http://website)\n");
    }
    #[test]
    fn test_02_badge_no_git_no_arg() {
        let v = create_struct_for_test();
        let badges = as_shield_badges(&v, "", true, false);

        assert_eq!(badges,"[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-11-green.svg)]()\n[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-22-blue.svg)]()\n[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-33-purple.svg)]()\n[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-55-yellow.svg)]()\n[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-44-orange.svg)]()\n");
    }
    #[test]
    fn test_03_badge_with_git_no_arg() {
        let v = create_struct_for_test();
        let badges = as_shield_badges(&v, "", false, true);

        assert_eq!(badges,"[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-11-green.svg)](https://github.com/LucianoBestia/lmake_lines_of_code/)\n[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-22-blue.svg)](https://github.com/LucianoBestia/lmake_lines_of_code/)\n[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-33-purple.svg)](https://github.com/LucianoBestia/lmake_lines_of_code/)\n[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-55-yellow.svg)](https://github.com/LucianoBestia/lmake_lines_of_code/)\n[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-44-orange.svg)](https://github.com/LucianoBestia/lmake_lines_of_code/)\n");
    }
}
