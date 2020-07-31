// count_lines_mod.rs
//! Module with fn to count rust lines.

use crate::utilsmod::*;
use anyhow;

#[allow(unused_imports)]
use ansi_term::Colour::{Green, Yellow};
use regex::Regex;
use serde_derive::Deserialize;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{env, fs, path::Path};
use unwrap::unwrap;

#[derive(Deserialize)]
struct CargoToml {
    workspace: Option<Workspace>,
}

#[derive(Deserialize)]
struct Workspace {
    members: Vec<String>,
}

#[derive(Default, Debug)]
/// Struct that contains 4 types of lines count: code, doc comments, comments, test and examples.
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

use crate::AppObject;
use crate::TraitCountLines;

impl TraitCountLines for AppObject {
    /// Returns the struct LinesOfCode for 4 types of lines:
    /// code, doc comments, comments, test and examples.
    /// Automatically detects if this is a workspace or single rust project.
    ///
    /// ## Example
    ///
    /// ```
    /// use lmake_lines_of_code::*;
    ///
    /// let app = AppObject{};
    /// let v = app.workspace_or_project_count_lines();
    /// dbg!(&v);
    /// ```
    fn workspace_or_project_count_lines(&self) -> LinesOfCode {
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
                let v = Self::one_project_count_lines(&current_dir.join(member));
                lines_of_code.src_code_lines += v.src_code_lines;
                lines_of_code.src_doc_comment_lines += v.src_doc_comment_lines;
                lines_of_code.src_comment_lines += v.src_comment_lines;
                lines_of_code.tests_lines += v.tests_lines;
                lines_of_code.examples_lines += v.examples_lines;
            }
        } else {
            lines_of_code = Self::one_project_count_lines(&current_dir);
        }
        // return
        lines_of_code
    }
    /// Return the string for link for badges like: https://github.com/LucianoBestia/lmake_lines_of_code/.  
    /// Get the output string after $ git remote -v.  
    /// Then finds out the link to the repository with regex.  
    /// Returns empty string if something goes wrong: no git, no remote,...  
    fn process_git_remote(&self) -> String {
        let output = match self.git_remote_output() {
            Ok(s) => s,
            Err(e) => {
                println!("{}", e);
                return "".to_string();
            }
        };
        match self.regex_capture(output) {
            Ok(s) => return s,
            Err(e) => {
                println!("{}", e);
                return "".to_string();
            }
        }
    }
}

impl AppObject {
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
    pub fn git_remote_output(&self) -> anyhow::Result<String> {
        let output = std::process::Command::new("git")
            .arg("remote")
            .arg("-v")
            .output()?;

        let output = String::from_utf8(output.stdout)?;
        println!("output: {}", &output);
        // return
        Ok(output)
    }
    /// returns a Result.
    /// in the case of error the calling fn will return empty string.
    pub fn regex_capture(&self, output: String) -> anyhow::Result<String> {
        // on Github actions they don't use ssh, but https, I need to check that also
        // I test my regex on https://regex101.com/
        // regex capture 3 groups: website, user_name and repo_name
        // "origin  git@github.com:LucianoBestia/lmake_lines_of_code.git (fetch)"
        // origin    https://github.com/LucianoBestia/lmake_lines_of_code (fetch)
        println!("{}", &output);
        let reg = Regex::new(
            r#"origin\s*(?:https://)?(?:git@)?([^:/]*?)[:/]([^/]*?)/([^. ]*?)(?:\.git)?\s*\(fetch\)"#,
        )?;
        let cap = reg
            .captures(&output)
            .ok_or(anyhow::anyhow!("Error: reg.captures is None"))?;
        // dbg!(&cap);

        // indexing can panic, but I would like it to Error
        anyhow::ensure!(
            cap.len() == 4,
            "Error: cap len is not 4, because there are 4 capture groups in regex."
        );
        return Ok(format!("https://{}/{}/{}/", &cap[1], &cap[2], &cap[3]));
    }
    /// Returns a string with the code for a markdown table with count of lines.
    ///
    /// Some websites render a beautiful table, but others render ugly tables.
    /// Use badges instead.
    ///
    /// ## Example
    ///
    /// ```
    /// use lmake_lines_of_code::*;
    ///
    /// let app = AppObject{};
    /// let v = app.workspace_or_project_count_lines();
    /// let badges = app.to_string_as_md_table(&v);
    ///
    /// println!("{}", badges);
    /// ```
    pub fn to_string_as_md_table(&self, v: &LinesOfCode) -> String {
        // I added an empty row to have the next row with different color from the header.
        format!(
            "
| src code | doc comments | comments | examples | tests |
| :------: | :----------: | :------: | :------: | :---: |
|  lines   |     lines    |   lines  |   lines  | lines |
| {:^8   } | {:^12      } | {:^8   } | {:^8   } | {:^5} |

",
            v.src_code_lines,
            v.src_doc_comment_lines,
            v.src_comment_lines,
            v.examples_lines,
            v.tests_lines
        )
    }
    /// Returns a string with the markdown code for 4 shield badges.
    ///
    /// Every badge has the link to the url given as first CLI argument
    /// or automatically finds out the github git remote repository url.
    ///
    /// ## Example
    ///
    /// ```
    /// use lmake_lines_of_code::*;
    ///
    /// let app = AppObject{};
    /// let v = app.workspace_or_project_count_lines();
    /// let badges = app.to_string_as_shield_badges(&v,"");
    ///
    /// println!("{}", badges);
    /// ```
    pub fn to_string_as_shield_badges(&self, v: &LinesOfCode, link: &str) -> String {
        println!("to_string_as_shield_badges() start");

        let src_code_lines = format!(
            "[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-{}-green.svg)]({})",
            v.src_code_lines, link
        );
        let src_doc_comment_lines = format!("[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-{}-blue.svg)]({})",v.src_doc_comment_lines,link);
        let src_comment_lines = format!(
        "[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-{}-purple.svg)]({})",
        v.src_comment_lines, link
    );
        let example_lines = format!(
        "[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-{}-yellow.svg)]({})",
        v.examples_lines, link
    );
        let tests_lines = format!(
            "[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-{}-orange.svg)]({})",
            v.tests_lines, link
        );
        //return
        format!(
            "{}\n{}\n{}\n{}\n{}\n",
            src_code_lines, src_doc_comment_lines, src_comment_lines, example_lines, tests_lines
        )
    }
}
impl LinesOfCode {
    pub fn new(
        src_code_lines: usize,
        src_doc_comment_lines: usize,
        src_comment_lines: usize,
        tests_lines: usize,
        examples_lines: usize,
    ) -> LinesOfCode {
        LinesOfCode {
            src_code_lines,
            src_doc_comment_lines,
            src_comment_lines,
            tests_lines,
            examples_lines,
        }
    }
}
