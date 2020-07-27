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
//! Just copy/paste it into README.md.  
//!
// endregion: lmake_md_to_doc_comments include README.md A //!

// region: Clippy
#![deny(unused_must_use)]

// CONS: Unnecessary code.
// PROS: more readable without knowing that the type is bool.
#[allow(clippy::bool_comparison)]
// endregion: Clippy
mod count_lines_mod;
mod readme_include_mod;
mod utilsmod;

pub use count_lines_mod::*;
pub use readme_include_mod::*;
pub use utilsmod::*;
