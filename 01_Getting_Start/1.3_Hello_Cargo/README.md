# Hello, Cargo!

* Cargo is Rust's build system and package manager.
* Cargo handles a lot of tasks for you, such as building your code, downloading the libraries your code depends on, and building those libraries.
* When we write more complex code, we will be needed more libraries, then in that case cargo manages all of that.
* we can check cargo version with using : `cargo --version`

## Create a project with cargo : 
* `cargo new Project_Name`, Example : `cargo new hello_world`
* This will create a directory hello_world and project hello_world
* when we go inside the directory hello_world, we will be having many files.
* Inside the project folder, we will have `src` dir & `Cargo.toml` file.
* `NOTE` : It has also initialized a new Git repo alongside with a `.gitignore` file. Git file won't be generated if we run `cargo new` within an existing Git Repo, we can override this behavior by using the `cargo new -vcs=git`.
* Git is popular version control system, we can change the version control by using `--vcs` flag.
*  Run `cargo new --help` to see the available options. 

### Cargo.toml

```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

* This is the `TOML` (Tom's Obvious, Minimal Language) format, which is Cargo's configuration format.
* The first line [package], is selection heading that indicates that the following statements are configuration a package. As we add more information to this file, we'll add other sections.
* The last line [dependencies], is the start of a section for you to list your project's dependencies. In Rust, packages of code are referred to as crates. We Won't need any other crate for this project, but we will in the first project in Chapter 2, so we'll use this dependencies section then.

## Building & Running a Cargo Project : 

* From the `hello_world`, we can build our code with following command : 
  
```sh
$ cargo build
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 2.85 secs
```
* This command create an executable file in `target/debug/hello_wold` directory, because the default build is a debug build, Cargo puts the binary in a directory named debug.

* We can run the executable with using this command : 
```sh
$ ./target/debug/hello_cargo # or .\target\debug\hello_cargo.exe on Windows
Hello, world!
```
* Running `cargo build` for first time also causes Cargo to create a new file at the top level: `Cargo.lock`. This file keeps track of the exact version of dependencies in your project. This project doesn't have dependencies, so the file is bit sparse. You won't ever need to change this file manually; Cargo manages it's contents for you.
  
* we can run it with `./target/debug/hello_world` or we can use `cargo run` also, it will compile the code & then run the code.
* Using `cargo run` is more convenient than having to remember to run cargo build and then use the path.

```sh
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/hello_cargo`
Hello, world!

```
## Building for release : 

* When our project is finally ready for release, we can use `cargo build --release` to compile it with optimization. 
* This command will create an executable in `target/release` instead of `target/debug`. 
* By turning them on lengthens the time it takes for your program to compile. This is why there are two different profile; one for development, when you want to rebuild quickly and often, and another for building the final program you'll give to a user that won't be rebuilt repeatedly and that will run as fast as possible. if you're benchmarking your code's running time, be sure run `cargo build --release` and benchmark with the executable in `target/release`.

## Cargo Convention : 

* With simple projects, `Cargo` doesn't provide a lot of value over just using `rustc`, but it will prove it's worth as your program become more intricate. Once program grows to multiple files or need a dependency, it's much easier to let Cargo coordinate the build.




## Summary : 
* We can create a project using `cargo new`.
* We can build a project using `cargo build`.
* We can build and run a project in one step using `cargo run`.
* We can build a project without producing a binary to check for errors using cargo check.
* Instead of saving the result of the build in the same directory as our code, cargo stores it in `target/debug` directory.
