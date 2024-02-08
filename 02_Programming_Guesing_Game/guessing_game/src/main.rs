fn main() {
    let word = String::from("Hello");

    // Immutable closure with `move` keyword
    let print_str = move || {
        println!("word inside = {}", word);
    };

    // Calling the closure
    print_str();

    // Attempting to use `word` outside of the closure will result in a compilation error
    // println!("word outside = {}", word); // This line will cause a compilation error
}
