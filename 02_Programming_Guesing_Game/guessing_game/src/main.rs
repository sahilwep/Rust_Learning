fn main(){
    
    let random = 100;

    {
        println!("random variable before shadowing in inner block = {}", random);

        // this deceleration shadows the outer random variable
        let random = "abc";
        
        println!("random after shadowing in inner block = {}", random);
        
    }
    // end of the inner block

    println!("random after shadowing in outer block = {}", random);
}