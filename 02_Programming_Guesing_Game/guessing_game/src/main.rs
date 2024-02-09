fn main(){
    let mut word = String::new();

    println!("Original String  = {}", word);
    
    // append a string to the word variable
    word.push_str("Hello World!");

    println!("Changed String  = {}", word);
}