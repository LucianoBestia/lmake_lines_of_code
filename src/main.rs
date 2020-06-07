// region: lmake_readme include "readme.md" //! A
//! # lmake_lines_of_code  
//! 
//! version: 2020.607.1321  date: 2020-06-07 authors: Luciano Bestia  
//! **Lines of code for Rust projects**
//! 
//! 
//! | src code | doc comments | comments | examples | tests |
//! | :------: | :----------: | :------: | :------: | :---: |
//! |      |            |        |         |     |
//! |   233    |      73      |    72    |    0     |   0   |
//! 
//! ## Lines of code for Rust projects
//! 
//! Lines of code are not a "perfect" measurement of anything.\
//! Anybody can write a very big number of lines of useless code and comments.\
//! But for 95% of the cases they are good enough.\
//! Most of the developers use some "standard" coding practices and that is quantifiable and comparable.\
//! \
//! The `src_code_lines` is the most important count.\
//! That is actual code written for that project without  doc comments, comments, unit tests, integration tests and examples.\
//! Sometimes is great to see a big number here. It means there was a lot of work invested. But other times we want to see a small number. It means the developer understands the problem very well and don't try to solve anything outside that scope.\
//! \
//! The `src_doc_comment_lines` counts doc comments. They will eventually become docs. The count of lines shows how many documentation is written.\
//! \
//! The `src_comment_lines` counts code comments. Code comments are important to understand the code. The count of lines shows how understandable is the code.\
//! \
//! The `tests_lines` counts lines in tests and shows how good is the code tested. Here are the unit tests and integration test combined.\
//! \
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
//! I will ignore the block comments. They are usually NOT used for comments, but to temporarily disable a piece of code. So I count this as code and not comments.\
//! \
//! The src/\*.rs file can contain unit tests that start with #[cfg(test)]. I assume that these are always at the end of the file. There should not be any normal code after #[cfg(test)], only tests.\
//! \
//! All other files: md, toml, html, js, ... are not counted.  
//! 
//! ### Workspace
//! 
//! Workspaces have member projects, that are written in cargo.toml. The program counts lines of every project and sums them together.  
//! 
//! ## Install
//! 
//! `cargo install lmake_lines_of_code`  
//! 
//! ## Development
//! 
//! Documentation:\
//! <https://lucianobestia.github.io/lmake_lines_of_code>\
//! List of prepared make tasks for development: build, run, doc, publish,...\
//! `clear; cargo make`  
//! 
//! ## cargo crev reviews and advisory
//! 
//! It is recommended to always use [cargo-crev](https://github.com/crev-dev/cargo-crev)\
//! to verify the trustworthiness of each of your dependencies, including this one.\
//! `cargo crev verify`\
//! Please, spread this info.\
//! On the web use url to read crate reviews example:\
//! <web.crev.dev/rust-reviews/crate/num-traits/>  
// endregion: lmake_readme include "readme.md" //! A

// region: Clippy
#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    // variable shadowing is idiomatic to Rust, but unnatural to me.
    clippy::shadow_reuse,
    clippy::shadow_same,
    clippy::shadow_unrelated,

)]
#![allow(
    // library from dependencies have this clippy warnings. Not my code.
    // Why is this bad: It will be more difficult for users to discover the purpose of the crate, 
    // and key information related to it.
    clippy::cargo_common_metadata,
    // Why is this bad : This bloats the size of targets, and can lead to confusing error messages when 
    // structs or traits are used interchangeably between different versions of a crate.
    clippy::multiple_crate_versions,
    // Why is this bad : As the edition guide says, it is highly unlikely that you work with any possible 
    // version of your dependency, and wildcard dependencies would cause unnecessary 
    // breakage in the ecosystem.
    clippy::wildcard_dependencies,
    // Rust is more idiomatic without return statement
    // Why is this bad : Actually omitting the return keyword is idiomatic Rust code. 
    // Programmers coming from other languages might prefer the expressiveness of return. 
    // It’s possible to miss the last returning statement because the only difference 
    // is a missing ;. Especially in bigger code with multiple return paths having a 
    // return keyword makes it easier to find the corresponding statements.
    clippy::implicit_return,
    // I have private function inside a function. Self does not work there.
    // Why is this bad: Unnecessary repetition. Mixed use of Self and struct name feels inconsistent.
    clippy::use_self,
    // Cannot add #[inline] to the start function with #[wasm_bindgen(start)]
    // because then wasm-pack build --target web returns an error: export run not found 
    // Why is this bad: In general, it is not. Functions can be inlined across crates when that’s profitable 
    // as long as any form of LTO is used. When LTO is disabled, functions that are not #[inline] 
    // cannot be inlined across crates. Certain types of crates might intend for most of the 
    // methods in their public API to be able to be inlined across crates even when LTO is disabled. 
    // For these types of crates, enabling this lint might make sense. It allows the crate to 
    // require all exported methods to be #[inline] by default, and then opt out for specific 
    // methods where this might not make sense.
    clippy::missing_inline_in_public_items,
    // Why is this bad: This is only checked against overflow in debug builds. In some applications one wants explicitly checked, wrapping or saturating arithmetic.
    // clippy::integer_arithmetic,
    // Why is this bad: For some embedded systems or kernel development, it can be useful to rule out floating-point numbers.
    clippy::float_arithmetic,
    // Why is this bad : Doc is good. rustc has a MISSING_DOCS allowed-by-default lint for public members, but has no way to enforce documentation of private items. This lint fixes that.
    clippy::doc_markdown,
    // Why is this bad : Splitting the implementation of a type makes the code harder to navigate.
    clippy::multiple_inherent_impl,

    clippy::missing_docs_in_private_items,
)]
// endregion

// region: mod, extern and use statements
mod count_lines_mod;
mod utilsmod;

use count_lines_mod::*;

//use clap::*;
//use unwrap::unwrap;

#[allow(unused_imports)]
use ansi_term::Colour::{Green, Red, Yellow};
//use ansi_term::Style;
use clap::App;
use std::env;
// endregion

#[allow(clippy::print_stdout, clippy::integer_arithmetic)]
/// The program starts here.
fn main() {
    // this function is different for Windows and for Linux.
    // Look at the code of this function (2 variations).
    enable_ansi_support();

    // define the CLI input line parameters using the clap library
    let _arguments = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .get_matches();
    //let v = one_project_count_lines();
    let v = workspace_or_project_count_lines();
    println!("{}", as_md_table(v));
}

// region: different function code for Linux and Windows
#[cfg(target_family = "windows")]
/// only on windows "enable ansi support" must be called
pub fn enable_ansi_support() {
    let _enabled = ansi_term::enable_ansi_support();
}

#[cfg(target_family = "unix")]
//on Linux "enable ansi support" must not be called
pub fn enable_ansi_support() {
    // do nothing
}
// endregion
