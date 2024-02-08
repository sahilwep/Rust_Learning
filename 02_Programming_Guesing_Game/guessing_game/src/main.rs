fn main() {
    let x = 10; // Variable to capture

    // Closure with `move` (owns `x`)
    let closure_with_move = move || println!("x is: {}", x); // Can be called later, even after `x` is gone

    closure_with_move();

    println!("access value after closure with move = {}", x); 
}