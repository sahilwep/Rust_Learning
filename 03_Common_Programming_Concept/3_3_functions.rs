/*
// function : `fn` keyword use for deceleration

// Statements & Expressions
    function bodies are made up of a series of statements optionally ending in an expression. 
*/

fn main(){

    println!("Sum: {}", add(1, 5));
}

fn add(x: i32, y: i32) -> i32 {
    // return x+y;      // we can also write this.
    x+y
}