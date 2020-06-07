# lmake_loc  

[comment]: # (lmake_readme cargo.toml data start)


[comment]: # (lmake_readme cargo.toml data end)

Work in progress.\
Not yet prepared for workspaces - multi-project !  

## Lines of code for Rust projects

Lines of code are not a "perfect" measurement of anything.\
Anybody can write a very big number of lines of useless code and comments.\
But for 95% of the cases they are good enough.\
Most of the developers use some "standard" coding practices and that is quantifiable and comparable.\
\
The most important count is `lines of code`.\
That is actual code written for that project without comments, doc comments, unit tests, integration tests and examples.\
Sometimes is great to see a big number here. It means there was a lot of work invested. But other times we want to see a small number. It means the developer understands the problem very well and don't try to solve anything outside that scope.\
\
The doc comments will eventually become docs. The count of lines shows how many documentation is written.\
\
The comments are important to understand the code. The count of lines shows how understandable is the code.\
\
The count of lines in tests shows how good is the code tested.\
\
The count of lines in examples shows how good is explained how to use the code.  

## Folder and file structure

The folder structure of a single Rust project is simple.\
The project starts in the folder that contains cargo.toml.\
The /src/ folder contains all the rust \*.rs files.\
The /tests/ folder contains integration tests.\
The /examples/ folder contains examples.\
Inside a rs file the doc comment line start with /// or //!.\
The normal comments start with // or /!.\
I will ignore the block comments. They are usually NOT used for comments, but to temporarily disable a piece of code. So I count this as code and not comments.\
\
The src/\*.rs file can contain unit tests that start with #[cfg(test)]. I assume that these are always at the end of the file. There should not be any normal code after #[cfg(test)], only tests.\
\
All other files: md, toml, html, js, ... are not counted.  

## Install

`cargo install lmake_loc`  

## Development

Documentation:\
<https://lucianobestia.github.io/lmake_loc>\
List of prepared make tasks for development: build, run, doc, publish,...\
`clear; cargo make`  

## cargo crev reviews and advisory

It is recommended to always use [cargo-crev](https://github.com/crev-dev/cargo-crev)\
to verify the trustworthiness of each of your dependencies, including this one.\
Please, spread this info.\
On the web use url to read crate reviews example:\
<web.crev.dev/rust-reviews/crate/num-traits/>  
