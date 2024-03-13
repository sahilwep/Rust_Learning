/*
// Understanding Ownership

*/


fn main() {
    let s: &str = "sahilwep"; // this string literals, which is immutable in nature
    let mut name = String::from("Sahil");       // this is String object, which is growable in nature & allocated in heap

    {
        let value = String::from("any value");  // scope starts from here for variable value
        // do stuff with value
    }   // scope ends here for variable value

    
    // variable and data interacting with move

    

}