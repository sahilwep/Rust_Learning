fn main(){
    // scope of the outer code block
    let outer_var = 32;

    
    {
        // scope of inner code block
        let inner_var = 92;
        println!("inner var = {}", inner_var);
        println!("outer var = {}", outer_var);
    }
    // end of the inner code block
    println!("outer var = {}", outer_var);
}