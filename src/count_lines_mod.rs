//! count_lines_mod.rs

use crate::utilsmod::*;

#[allow(unused_imports)]
use ansi_term::Colour::{Green, Yellow};
use std::{env, path::Path};
use std::fs::File;
use std::io::{BufRead, BufReader};
use unwrap::unwrap;

#[derive(Clone, Debug, Default)]
pub struct LinesOfCode {
    /// lines in all rs files
    pub all_lines:usize,
    /// all lines in examples files
    pub examples_all_lines:usize,
    /// all lines in tests files
    pub tests_all_lines:usize,
    /// all lines in srs files
    pub src_all_lines:usize,
    /// lines with code in srs files
    pub src_code_lines:usize,
    /// lines with doc_comments in srs files
    pub src_doc_comment_lines:usize,
    /// lines with comments in srs files
    pub src_comment_lines:usize,
    /// line is unit tests
    pub src_test_lines:usize,
}

pub enum SpecialFolder{
    src,
    tests,
    examples,
}

pub fn count_lines() -> LinesOfCode{
    let mut lines_of_code=LinesOfCode::default();

    let current_dir = unwrap!(env::current_dir());
    let current_dir = unwrap!(current_dir.to_str());
    println!("current_dir: {}", Yellow.paint(current_dir));

    let files = unwrap!(traverse_dir_with_exclude_dir(
        Path::new(current_dir),
        "/*.rs",
        // avoid big folders and other folders with *.crev
        &vec!["/.git".to_string(), "/target".to_string(), "/docs".to_string()]
    ));
    // println!("{:#?}", files);
    for rs_file_name in files.iter(){
        dbg!(&rs_file_name);
        //if file inside src/ or tests/ or examples
        let special_folder = if rs_file_name.contains("/examples/"){
             SpecialFolder::examples
        } else if rs_file_name.contains("/tests/"){
            SpecialFolder::tests
        } else{
            SpecialFolder::src
        };
         // Open the file in read-only mode (ignoring errors).
        let file = File::open(rs_file_name).unwrap();
        let reader = BufReader::new(file);
        let mut is_unit_test=false;
        // Read the file line by line using the lines() iterator from std::io::BufRead.
        for (index, line) in reader.lines().enumerate() {
            lines_of_code.all_lines +=1;
            match special_folder {
                SpecialFolder::examples=> {
                    lines_of_code.examples_all_lines +=1;
                }
                SpecialFolder::tests=> {
                    lines_of_code.tests_all_lines +=1;
                }
                SpecialFolder::src=> {
                    lines_of_code.src_all_lines +=1;
                    let line = line.unwrap(); // Ignore errors.
                    let line = line.trim_start();
                    if line.starts_with("///") || line.starts_with("//!"){
                        lines_of_code.src_doc_comment_lines +=1;
                    } else if line.starts_with("//") || line.starts_with("/!"){
                        lines_of_code.src_comment_lines +=1;
                    } else if line.starts_with("#[cfg(test)]"){
                        is_unit_test = true;
                    } else if is_unit_test==true{
                        lines_of_code.src_test_lines += 1;
                    } else{
                        lines_of_code.src_code_lines += 1;
                    }
                }
            }
        }
    }
    // return
    lines_of_code
}
