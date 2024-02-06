
// function to add two numbers in Rust
fn add(a: i32, b: i32) -> i32 {
    let sum = a + b;
    return sum;
}

fn main(){
    let a = 5;
    let b = 10;
    
    // function call inside println
    println!("Sum of a and b = {}", add(a,b));
}