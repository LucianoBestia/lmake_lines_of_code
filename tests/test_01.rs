// test_01.rs
use lmake_lines_of_code::*;

// Before test, change this const to your repo link and actual lines of code of this project.
// This is read from the CLI $ git remote -v.
// If you don't use git or git remote, modify to empty string.
const REPO_LINK: &str = "https://github.com/LucianoBestia/lmake_lines_of_code/";
// the count of lines of this project will change with time. Keep it actual for testing.
const LINES_OF_CODE: LinesOfCode = LinesOfCode {
    src_code_lines: 365,
    src_doc_comment_lines: 269,
    src_comment_lines: 52,
    tests_lines: 91,
    examples_lines: 0,
};

#[test]
/// change the const REPO_LINK accordingly to your system (does it use git, what repo remote link?)
/// automatically finds GitHub git remote repository
/// if anything goes wrong (no git, no remote) returns empty_string
fn test_01_process_git_remote() {
    let app = AppObject::new();
    let s = app.process_git_remote();
    assert_eq!(&s, REPO_LINK);
}

#[test]
/// tests the badge md code
fn test_02_badge() {
    let app = AppObject::new();
    let v = LinesOfCode::new(11, 22, 33, 44, 55);
    let badges = app.to_string_as_shield_badges(&v, "http://website");
    assert_eq!(badges,"[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-11-green.svg)](http://website)\n[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-22-blue.svg)](http://website)\n[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-33-purple.svg)](http://website)\n[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-55-yellow.svg)](http://website)\n[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-44-orange.svg)](http://website)\n");
}

#[test]
/// test the text to include
/// depends on the lines of code of this project
/// but not on the $ git remove -v result
fn test_03_text_to_include_with_link_arg() {
    let app = AppObject::new();
    let t = app.text_to_include("http://website");
    let prediction =format!("[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-{l1}-green.svg)](http://website)\n[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-{l2}-blue.svg)](http://website)\n[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-{l3}-purple.svg)](http://website)\n[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-{l4}-yellow.svg)](http://website)\n[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-{l5}-orange.svg)](http://website)\n", 
l1=LINES_OF_CODE.src_code_lines,
l2=LINES_OF_CODE.src_doc_comment_lines,
l3=LINES_OF_CODE.src_comment_lines,
l4=LINES_OF_CODE.examples_lines,
l5=LINES_OF_CODE.tests_lines);
    assert_eq!(t, prediction);
}

#[test]
/// test the text to include
/// depends on the lines of code of this project
/// and depends on the $ git remove -v result
/// change the const REPO_LINK to the string you got from git
fn test_04_text_to_include_with_git_remote() {
    let app = AppObject::new();
    let t = app.text_to_include("");
    let prediction = format!("[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-{l1}-green.svg)]({l})\n[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-{l2}-blue.svg)]({l})\n[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-{l3}-purple.svg)]({l})\n[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-{l4}-yellow.svg)]({l})\n[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-{l5}-orange.svg)]({l})\n", l=REPO_LINK,
    l1=LINES_OF_CODE.src_code_lines,
    l2=LINES_OF_CODE.src_doc_comment_lines,
    l3=LINES_OF_CODE.src_comment_lines,
    l4=LINES_OF_CODE.examples_lines,
    l5=LINES_OF_CODE.tests_lines);
    assert_eq!(t, prediction);
}

#[test]
/// tests if it writes correctly to README.md
/// if the
fn test_05_include_in_readme() {
    use std::fs;
    let app = AppObject::new();
    // first change the file README.md, so we can test the change.
    app.text_to_include("dummy");

    // testing the main() method
    let text_to_include = "\n".to_string() + &app.main("") + "\n";
    // read md file and find included text
    let start_delimiter = "[comment]: # (lmake_lines_of_code start)";
    let end_delimiter = "[comment]: # (lmake_lines_of_code end)";
    let file_name = "README.md";
    let mut included = String::new();
    let readme_content = fs::read_to_string(file_name).unwrap();
    let mut pos_start = readme_content.find(start_delimiter).unwrap();
    pos_start += start_delimiter.len();
    let pos_end = readme_content.find(end_delimiter).unwrap();
    included.push_str(&readme_content[pos_start..pos_end]);
    assert_eq!(included, text_to_include);
}
