/*
// for compilation : 
    $ rustc main.rs

// for running :
    $ ./main

// Notes : 
    * println! calls rust macro. if it had called a function instead, it would be entered as println (without the !), it will discuss later on.
    * Wherever we will use ! it means we are calling macro instead of normal function.
    * line is end with ;

*/

fn main(){
    println!("Hello World!");
} 