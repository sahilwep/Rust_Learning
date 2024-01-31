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

## Creating program : 

```rust
use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed : {}", guess);
}

```
* For creation of variable we use `let` keyword
* we specify the `mut` keyword to make our variable mutable
* Mutable : 
    * In rust by default the variable are immutable, means once we assign the value we can't change value of it.
    * to make it mutable we use `mut` keyword.
* Then we use `name` of the variable & then we assign `=` after that we use `String::new()`
* We use `=` to bind the string to the mut variable `guess`.
* String::new : is a function that returns new instances of string.
* String is string type provided by the standard library that is a growable
* `new` is associate with string function of the string type. An associated function is a function that's implemented on a type. in this case String.
* This new function creates a new, empty sting.
* We will find new function to many types because it's a common name for a function that makes a new value of some kind.
* In full : `let mut guess = String::new();`

* We can make variables like : 
  
```rust
let mut apple = 5;  // mutable variable
let mango = 2;  // immutable variable

```

### Receiving User Input : 

```Rust
io::stdin()
    .read_line(&mut guess)

```


