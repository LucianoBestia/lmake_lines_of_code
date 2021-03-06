// region: lmake_md_to_doc_comments include README.md A //!
//! # lmake_lines_of_code  
//!
//! ***version: 1.1.7  date: 2020-08-22 authors: Luciano Bestia***  
//! **Lines of code for Rust projects**
//!
//! [![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-379-green.svg)](https://github.com/LucianoBestia/lmake_lines_of_code/)
//! [![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-269-blue.svg)](https://github.com/LucianoBestia/lmake_lines_of_code/)
//! [![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-56-purple.svg)](https://github.com/LucianoBestia/lmake_lines_of_code/)
//! [![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/LucianoBestia/lmake_lines_of_code/)
//! [![Lines in tests](https://img.shields.io/badge/Lines_in_tests-142-orange.svg)](https://github.com/LucianoBestia/lmake_lines_of_code/)
//!
//! [![crates.io](https://meritbadge.herokuapp.com/lmake_lines_of_code)](https://crates.io/crates/lmake_lines_of_code) [![Documentation](https://docs.rs/lmake_lines_of_code/badge.svg)](https://docs.rs/lmake_lines_of_code/) [![crev reviews](https://web.crev.dev/rust-reviews/badge/crev_count/lmake_lines_of_code.svg)](https://web.crev.dev/rust-reviews/crate/lmake_lines_of_code/) [![Lib.rs](https://img.shields.io/badge/Lib.rs-rust-orange.svg)](https://lib.rs/crates/lmake_lines_of_code/) [![Licence](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/LucianoBestia/lmake_lines_of_code/blob/master/LICENSE) [![Rust](https://github.com/LucianoBestia/lmake_lines_of_code/workflows/RustAction/badge.svg)](https://github.com/LucianoBestia/lmake_lines_of_code/)
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
//! If you don't use git add the link of repository as CLI argument:  
//! `lmake_lines_of_code http://repo-website`  
//!
// endregion: lmake_md_to_doc_comments include README.md A //!

// region: mod, extern and use statements
#[allow(unused_imports)]
use ansi_term::Colour::{Green, Red, Yellow};
//use ansi_term::Style;
//use unwrap::unwrap;
use std::env;
// endregion

use lmake_lines_of_code::*;

#[allow(clippy::print_stdout, clippy::integer_arithmetic)]
/// The program starts here.
fn main() {
    // this function is different for Windows and for Linux.
    // Look at the code of this function (2 variations).
    enable_ansi_support();

    // define the CLI input line parameters using the clap library
    let arguments = clap::App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(clap::Arg::with_name("link").help("Link to include in shield badge. If not defined, the git remote repository will be used."))
        .get_matches();

    let link = arguments.value_of("link").unwrap_or("");

    println!("---- {} start ----", Green.paint(env!("CARGO_PKG_NAME")));
    let app = AppObject::new();
    let _text_to_include = app.main(link);
    println!("---- {} end ----", Green.paint(env!("CARGO_PKG_NAME")));
}

// region: different function code for Linux and Windows
#[cfg(target_family = "windows")]
/// only on windows "enable ansi support" must be called
fn enable_ansi_support() {
    let _enabled = ansi_term::enable_ansi_support();
}

#[cfg(target_family = "unix")]
//on Linux "enable ansi support" must not be called
fn enable_ansi_support() {
    // do nothing
}
// endregion
