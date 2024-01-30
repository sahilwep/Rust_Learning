# Programming a Guessing Game : 

## TOML Configuration : 
* TOML : Tom's Obvious Minimal Language
* This file contains configuration files.

```TOML
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]

```

* First line `[package]`, is a section heading that indicate the following statements are configuration a package.
* Next three lines set the configuration information Cargo need to compile to complete your program: the name, the version, and the edition of rust to use.
* The last line, `[dependencies]`, is start of a section for you to list any of your project's dependencies. In rust package of code are referred as crates.