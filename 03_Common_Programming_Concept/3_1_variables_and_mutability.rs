fn main(){

    // Variable : 
    let x = 5;  // immutable variable : we can't change the value once it declare.
    println!("x = {x}");

    let mut y = 7;  // mutable variable : we can change the value later on.
    println!("y = {y}");
    y = 8;
    println!("y = {y}");


    // Constants : like immutable variable, constants are value that are bound to a name and are not allowed to change
    const PI: f32 = 3.14;


    // Shadowing : 
    let _val = 10;
    let _val = _val + 1;
    {
        let _val = _val * 2;
        println!("The value of `_val` in inner scope is: {x} ");
    }
    println!("The value of `_val` is: {x} ");

    let space = " ";
    println!("{} ", space.len());

}