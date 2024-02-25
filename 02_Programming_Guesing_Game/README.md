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

* We can receive user input by using `stdin()` method from `std::io` library, as we have import in first line, if we couldn't import this, we still use them.

```Rust
io::stdin()
    .read_line(&mut guess)
```

* `.read_line(&mut guess)` calls the **read_line** method on the standard input handle to get input from the user. We're also passing `&mut guess` as the argument to **read_line** to tell it what string to store the user input in. The full job of **read_line** is to take whatever the user type into standard input and append that into a string.
* The `&` indicates that this argument is reference, which give you a way to let multiple parts of your code can access one piece of data without needing to copy that data into memory multiple times. Reference are complex features and one Rust's major advantages is how safe and easy it is to use reference. 

### Handling Potential Failure with Result

* We're still working on this line of code. We're now discussing a third line of the text, byt note that it's still part of a single logical line of code. The next part is this method:
```rust
    .except("Fail to read line");
```
* We could have written this code as:
```rust
io::stdin().read_line(&mut guess).expect("failed to read line");;
```
* However, one long line is difficult to read, so it's best to divide it. It's often wise to introduce a newline and other whitespace to help break up long line when we call a method with `.method_name()` syntax. Now let's discuss what this line does.

* As mentioned earlier, `read_line` puts whatever the user enters into the string we pass to it, but it also returns a `Result` value. `Result` is an **enumeration**, often called an enum, which is a type that can be in one of multiple possible states. We call each possible a variant. We will cover enums latter on.
* `Result` variants are : 
  * `Ok` : indicates operations are successful.
  * `Err` : means operation failed.

* An instance of `Result` has an `expect` **method** that we can call.

### Printing values with **println!** placeholder

```rust
println!("You guessed: {guess}");
```

* This line prints the string that now contains the user input, the `{}` set of curly brackets is a placeholder.

## Generating a Secret Number :

* We can use `rand` **crate** to generate random number.
* Remember that crate is a collection of Rust source code files. The project we've been building is a binary code, which is an executable. the `rand crate is a library crate, which contains code that is intended to be used in other program and can't be executed on it's own.
* We need to modify `Cargo.toml` file by adding `rand` crate inside it.
```rust
[dependencies]
rand = "0.8.5"
```
* When we include an external dependency, Cargo fetches the latest version of everything that dependency needs from the registry, which is a copy of data from **Crates.io**. Crates.io is where people in Rust ecosystem post their open source Rust projects for others to use.
* when we run `cargo build` after including crates in dependencies, cargo will download all the necessary crates from **Crates.io**.

### Ensuring Reproducible Builds with the Cargo.lock File

* Cargo has a mechanism that ensures you can rebuild the same artifact every time you or anyone else builds your code: Cargo will use only the versions of the dependencies you specified until you indicate otherwise. For example, say that next week version 0.8.6 of the `rand` crate comes out, and that version contains an important bug fix, but it also contains a regression that will break your code. To handle this, Rust creates the Cargo.lock file the first time you run `cargo build`, so we now have this in the guessing_game directory.

* When you build a project for the first time, Cargo figures out all the versions of the dependencies that fit the criteria and then writes them to the Cargo.lock file. When you build your project in the future, Cargo will see that the Cargo.lock file exists and will use the versions specified there rather than doing all the work of figuring out versions again. This lets you have a reproducible build automatically. In other words, your project will remain at 0.8.5 until you explicitly upgrade, thanks to the Cargo.lock file. Because the Cargo.lock file is important for reproducible builds, it’s often checked into source control with the rest of the code in your project.

### Updating a crate to Get New Version

```rust
$ cargo update
    Updating crates.io index
    Updating rand v0.8.5 -> v0.8.6
```
* This command will update crates to their new version.

* We can use `cargo doc --open` to read documentation about our projects.

### Comparing the Guess to the Secret Number

* We can use `std::cmp::Ordering` into scope from the standard library. The `Ordering` type is another enum has the variants `Less`, `Grater`, and `Equal`. These are the three outcomes that are possible when we compare two values.

* We can use the `match` expression to match the values. `match` expression is made up of **arms**. An arm consists of a pattern to match against, and the code that should be run if the value given to `match` fits that arm's pattern.

* We cna use this parse string into integer : 
```rust
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```

### Allowing Multiple Guesses with looping

* We can use loop keyword creates an infinite loop.
* Quitting After a correct Guess : we can use `break` statement to break out.

```rust
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){
    println!("Welcome to Guessing Game");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Enter your guessed number: ");    
        let mut guess = String::new();  // Create a string to take input.
        
        // Taking input
        io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");
    
        let guess: u32 = guess.trim().parse().expect("Please type a number!");  // parsing string to int only
        
        println!("You guessed: {}", guess); // printing gyess
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal =>  {
                println!("You win!");
                break;
            }
        }
    }      
}
```


### Handling invalid input : 

```rust
// --snip--
io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");

let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};

println!("You guessed: {}", guess);

// --snip--
```
* We switch from an `expect` call to a `match` expression from crashing on an error to handling the error. Remember that `parse` returns a `Result` type and `Result` is an enum that has the variants `Ok` and `Err`. We're using `match` expression here, as we did with the `Ordering` result of the `cmp` method.

* If `parse` is able to successfully turn the string into a number, it will return an `Ok` value that contains the resultant number. That `Ok` value will match the first arm’s pattern, and the `match` expression will just return the `num` value that `parse` produced and put inside the `Ok` value. That number will end up right where we want it in the new `guess` variable we’re creating.

* If `parse` is not able to turn the string into a number, it will return an `Err` value that contains more information about the error. The Err value does not match the `Ok(num)` pattern in the first `match` arm, but it does match the `Err(_)` pattern in the second arm. The underscore, `_`, is a catchall value; in this example, we’re saying we want to match all Err values, no matter what information they have inside them. So the program will execute the second arm’s code, `continue`, which tells the program to go to the next iteration of the `loop` and ask for another guess. So, effectively, the program ignores all errors that `parse` might encounter!

* Final code look like : 

```rust
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){
    println!("Welcome to Guessing Game");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {}", secret_number);    // uncomment this to know guessed number.

    let mut cnt = 0;

    // loop to iterate until user wins the game
    loop {
        println!("Enter your guessed number: ");    
        let mut guess = String::new();  // Create a string to take input.
        
        // Taking input
        io::stdin()
        .read_line(&mut guess)  // reading line
        .expect("failed to read line"); // handling failure 
        
        cnt = cnt + 1;  // counter

        // parsing string to integer
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, // handling error
            Err(_) => continue, // if error skip the guessing iteration.
        };
        
        println!("You guessed: {}", guess); // printing guessed number
        
        // match guess number with random generated number.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal =>  {
                println!("You win!");   // when user wins the loop will exit.
                break;
            }
        }
    }   
    println!("You have taken {} times to solve", cnt);
}
```