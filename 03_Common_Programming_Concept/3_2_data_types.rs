/*
 // Variable: 
    /*
        Scalar type : A scalar type represent a single value.
        Rust has four primary scalar types: integer, floating-point number, boolean, and characters.
        
    */
    // integer types: 
    // -> i = signed value, -ve & +ve numbers
    // -> u = unsigned value, only +ve numbers
    // Each variants stores numbers from -(2^(n-1)) to 2^(n-1)) inclusive, n = number of bits that variant uses.

// Integer Literals:
    // decimal : 98_222
    // hex     : 0xff
    // octal   : 0o77
    // binary  : 0b1111_0000
    // Byte(`u8` only) : b'A'

// `_` : this is visual separator to make the number easier to read.
    // 1_000 is same as 1000

// When we are not sure which type of integer to use ?
    -> use `i32` : the rust default type.
    //  The primary situation in which you'd use `isize` or `usize` is when indexing some sort of collection.

// Integer Overflow :
    -> When we compiling our code in debug mode : 
    -> let say we have use `u8` that has range between 0 and 255. if we try outer range value, Rust include check for integer overflow that cause your program to panic! at runtime if this behavior occurs.
    -> Rust uses the term `panicking` when a program exit with an error.
    
    -> When we are compiling our code in release mode, rust will not include check for integer overflow that causes panic.
    -> Instead, if overflow occurs, Rust performs two's complement wrapping. In short, value grater than the maximum value the type can hold `wrap around` to the minimum of the values the type can hold.
    -> in case of `u8` let say value 257, will become 1. The program won't panic, but the variable will have a value that probably ins't what we are expecting it to have.
    
    -> To explicitly handel the possibility of overflow, we can use these families of methods provided by the standard library for primitive numeric types:
        * wrap in all modes with the `wrapping_*` method, such as `wrapping_add`.
        * Return the `None` value if there is overflow with the `checked_*` methods.
        * Return the value and a boolean indicating whether there was overflow with the `overflowing_*` methods.
        * Saturate at the value's minimum or maximum values with the `saturating_*` methods.

// Compound Types: 
    compound type can group multiple value in one type. Rust has two primitive compound types : tuples and arrays.
    
    // tuple type : tuple can hold different types of data in single variable.
        // we can access tuple like : tup.index

    // array : array can hold collection of multiple value with same type.
        // syntax : 
            let array_name: [data_type; size] = [value1, value2, value3,.., valueN];
            leet array_name = [5, 1];   => [1, 1, 1, 1, 1]
            
*/

fn main(){

       let num1: i8 = 10; // represent 8bit signed value.
    let num_1: u8 = 10; // represent 8bit unsigned value.

    let num2: u32 = 1213;
    let num3: u64 = 123124;
    let num4: u128 = 22323434;
    let num5: isize = 12312312312;  // `usize` and `isize` types depend on the architecture of the computer your  program is running on, which is denoted in the table as `arch`:64 bits, if you're on a 64 bit architecture and 32 bits if you're on 32 bit architecture.
    
    println!("num1 = {num1} ");
    println!("num_1 = {num_1} ");
    println!("num2 = {num2} ");
    println!("num3 = {num3} ");
    println!("num4 = {num4} ");
    println!("num5 = {num5} ");
    
    // floating-point types: f32 & f64

    let float_val: f64 = 2.43;  // rust default floating value.
    let float_val_1: f32 = 3.9; // 32 bit size floating value.
    println!("float_val = {} ", float_val);
    println!("float_val_1 = {} ", float_val_1);
            
    // Boolean types : 

    let t = true;
    let f: bool = false;


    // Compound Types: 
    // tuple type
    let tup: (i32, f64, u8) = (500, 6.4, 1);    // tuple
    let (x, y, z) = tup;

    println!("y: {}", y);
    println!("y: {}", tup.1);   // we can pass the index : tuple.index, index start form `0`


    // array types:
    let a = [1, 2, 3, 4, 5];
    let arr: [i32; 5] =  [1, 12, 13, 14, 15];
    let b = [3, 5];     // equivalent to    `let a = [3, 3, 3, 3, 3];` 
    // accessing array elements : 
    let first = a[0]; 
    let second = a[1]; 
    println!("{first}, {second} ");

}   