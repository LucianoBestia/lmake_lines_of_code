// include_into_readme_md_mod.rs
//! Includes (writes, modifies) the shield badge code into README.md file.

#[allow(unused_imports)]
use ansi_term::Colour::{Green, Red, Yellow};
use std::fs;
use unwrap::unwrap;

/// Includes (writes, modifies) the shield badge code into README.md file.
///
/// ## Example
///
/// ```
/// use lmake_lines_of_code::*;
/// let app = AppObject::new();
/// let text_to_include = app.text_to_include("");
/// let v = include_into_readme_md(&text_to_include);
/// ```
pub fn include_into_readme_md(include_str: &str) {
    let start_delimiter = "[comment]: # (lmake_lines_of_code start)";
    let end_delimiter = "[comment]: # (lmake_lines_of_code end)";
    let file_name = "README.md";

    if let Ok(readme_content) = fs::read_to_string(file_name) {
        let mut new_readme_content = String::with_capacity(readme_content.len());
        if let Some(mut pos_start) = readme_content.find(start_delimiter) {
            pos_start += start_delimiter.len();
            if let Some(pos_end) = readme_content.find(end_delimiter) {
                new_readme_content.push_str(&readme_content[..pos_start]);
                new_readme_content.push_str("\n");
                new_readme_content.push_str(include_str);
                new_readme_content.push_str("\n");
                new_readme_content.push_str(&readme_content[pos_end..]);
                println!(
                    "include_into_readme_md write file: {}",
                    Green.paint(file_name)
                );
                unwrap!(fs::write(file_name, new_readme_content));
            }
        }
    }
}
