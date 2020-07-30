// region: lmake_md_to_doc_comments include README.md A //!
//! # lmake_lines_of_code  
//!
//! ***version: 1.1.5  date: 2020-07-27 authors: Luciano Bestia***  
//! **Lines of code for Rust projects**
//!
//! [![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-318-green.svg)](https://github.com/LucianoBestia/lmake_lines_of_code/)
//! [![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-196-blue.svg)](https://github.com/LucianoBestia/lmake_lines_of_code/)
//! [![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-54-purple.svg)](https://github.com/LucianoBestia/lmake_lines_of_code/)
//! [![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/LucianoBestia/lmake_lines_of_code/)
//! [![Lines in tests](https://img.shields.io/badge/Lines_in_tests-33-orange.svg)](https://github.com/LucianoBestia/lmake_lines_of_code/)
//!
//! [![crates.io](https://meritbadge.herokuapp.com/lmake_lines_of_code)](https://crates.io/crates/lmake_lines_of_code) [![Documentation](https://docs.rs/lmake_lines_of_code/badge.svg)](https://docs.rs/lmake_lines_of_code/) [![crev reviews](https://web.crev.dev/rust-reviews/badge/crev_count/lmake_lines_of_code.svg)](https://web.crev.dev/rust-reviews/crate/lmake_lines_of_code/) [![Lib.rs](https://img.shields.io/badge/Lib.rs-rust-orange.svg)](https://lib.rs/crates/lmake_lines_of_code/) [![Licence](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/LucianoBestia/lmake_lines_of_code/blob/master/LICENSE) [![Rust](https://github.com/LucianoBestia/lmake_lines_of_code/workflows/Rust/badge.svg)](https://github.com/LucianoBestia/lmake_lines_of_code/)
//!
//! ## Lines of code for Rust projects
//!
//! Lines of code are not a "perfect" measurement of anything.\
//! Anybody can write a very big number of lines of useless code and comments.\
//! But for 95% of the cases they are good enough.\
//! Most of the developers use some "standard" coding practices and that is quantifiable and comparable.  
//!
//! The `src_code_lines` is the most important count.\
//! That is actual code written for that project without  doc comments, comments, unit tests, integration tests and examples.\
//! Sometimes is great to see a big number here. It means there was a lot of work invested. But other times we want to see a small number. It means the developer understands the problem very well and don't try to solve anything outside that scope.  
//!
//! The `src_doc_comment_lines` counts doc comments. They will eventually become docs. The count of lines shows how many documentation is written.  
//!
//! The `src_comment_lines` counts code comments. Code comments are important to understand the code. The count of lines shows how understandable is the code.  
//!
//! The `tests_lines` counts lines in tests and shows how good is the code tested. Here are the unit tests and integration test combined.  
//!
//! The `examples_lines` counts lines in examples and shows how good is explained how to use the code.  
//!
//! ## Folder and file structure
//!
//! The folder structure of a single Rust project is simple.\
//! The project starts in the folder that contains cargo.toml.\
//! The /src/ folder contains all the rust \*.rs files.\
//! The /tests/ folder contains integration tests.\
//! The /examples/ folder contains examples.\
//! Inside a rs file the doc comment line start with /// or //!.\
//! The normal comments start with // or /!.\
//! I will ignore the block comments. They are usually NOT used for comments, but to temporarily disable a piece of code. So I count this as code and not comments.  
//!
//! The src/\*.rs file can contain unit tests that start with #[cfg(test)]. I assume that these are always at the end of the file. There should not be any normal code after #[cfg(test)], only tests.  
//!
//! All other files: md, toml, html, js, ... are not counted.  
//!
//! ### Workspace
//!
//! Workspaces have member projects, that are written in cargo.toml.\
//! The program counts lines of every project and sums them together.  
//!
//! ## Output
//!
//! The output is markdown text for a table and markdown text for shield badges.\
//! If the CLI is called with one argument:  
//! `lmake_lines_of_code http://website`  
//! This will be used for the link of all 4 shield badges.  
//! Else the app will try  
//! `git remote -v`  
//! to get the remote url.  
//! You can copy/paste it into README.md.  
//!
//! ## Include into README.md
//!
//! If the README.md file contains these markers:  
//!
//! 1. `[comment]: # (lmake_lines_of_code start)`  
//! 2. `[comment]: # (lmake_lines_of_code end)`  
//!
//! the CLI will include the shield badges code between them.  
//! It will erase the previous content.  
//! Use git diff to see the change.  
//!
//! ## Install and run
//!
//! `cargo install lmake_lines_of_code`  
//! Run in the Rust project folder, where cargo.toml is.  
//! If you use git:  
//! `lmake_lines_of_code`  
//! If you don't use git:  
//! `lmake_lines_of_code http://repo-website`  
//!
// endregion: lmake_md_to_doc_comments include README.md A //!

// region: Clippy
#![deny(unused_must_use)]

// CONS: Unnecessary code.
// PROS: more readable without knowing that the type is bool.
#[allow(clippy::bool_comparison)]
// endregion: Clippy
use mockall::predicate::*;
use mockall::*;

mod count_lines_mod;
mod readme_include_mod;
mod utilsmod;

pub use count_lines_mod::*;
pub use readme_include_mod::*;
pub use utilsmod::*;

/// An object to implement methods rather than functions.  
/// The methods are always defined in Traits, to be testable/mockable.  
/// Traits don't have access to fields, only to methods.  
pub struct AppObject {}

impl AppObject {
    /// Constructor of the object that has all the public methods.  
    /// No fields needed for now.  
    pub fn new() -> AppObject {
        AppObject {}
    }
    /// Runs all the public methods. It is called by the CLI main fn.
    pub fn main(&self, link: &str) -> String {
        let text_to_include = self.text_to_include(link);

        include_into_readme_md(&text_to_include);
        // return
        text_to_include
    }
    pub fn text_to_include(&self, link: &str) -> String {
        let v = self.workspace_or_project_count_lines();
        println!("{}", self.to_string_as_md_table(&v));

        let link = if link.is_empty() {
            self.process_git_remote()
        } else {
            link.to_string()
        };
        let text_to_include = self.to_string_as_shield_badges(&v, &link);
        println!("{}", &text_to_include);
        // return
        text_to_include
    }
}

/// Traits and methods must be used for the mocking library.
#[automock]
pub trait TraitCountLines {
    fn workspace_or_project_count_lines(&self) -> LinesOfCode;
    fn process_git_remote(&self) -> String;
}
