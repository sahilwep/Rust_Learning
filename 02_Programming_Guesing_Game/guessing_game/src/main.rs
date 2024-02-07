fn main(){
    
    let mut age = 100;

    {
        // shadowing by age variable
        let age = age;

        println!("age variable inner block = {}", age);
        // age goes out of scope
    }
    // end of the inner block

    // age variable is not frozen in outer block
    age = 3;

    println!("age variable outer block = {}", age);
}