use std::collections::HashMap;

fn main(){
    // create a new HashMap
    let mut fruits: HashMap<i32, String> = HashMap::new();

    // add key-value in a hashmap
    fruits.insert(1, String::from("Apple"));
    fruits.insert(2, String::from("orange"));
    
    println!("Length of fruit hashmap {} ", fruits.len());
}