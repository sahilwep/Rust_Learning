# Rust General Concepts : 

## Printing Output : 
* For printing we can use `print!()` or `println!()` macro.

```Rust
fn main(){
    println!("This is print statement!");

}
```
### Print variable : 

* for printing variable we will be using `{}` curly braces to fetch the variables.
* 

```Rust
fn main(){
    let age = 20;   // variable 
    let money = 100000;

    print!("your age is {} & you have {}$ in your account", age, money);    // we can fetch by order.
    print!("your age is {age} & you have {money}$ in your account"); // we can fetch value with just writing name of variable.
    print!("your age is {1} & you have {0}$ in your account", money, age);      // we can fetch value with using index.
    print!("Rust is fun! \n I love Learning Rust"); // we can use \n \t etc...
}
```
## Rust Variables & Mutability : 

```Rust
fn main(){

    let x = 1;  // immutable variable
    let mut y = 3;  // mutable variable 
    y = 5;
}
```
* By default rust create every variable as immutable, means once it's declare we can't change the value of it..

* With using `mut` Keyword during variable declaration to make our variable as mutable.
  
### Rules for naming Variables in Rust : 

* During Naming of variables we need to take care of special character, naming_Cases, numbers etc...

```rust 
// Variables must start with either letters or an underscore
    let age = 30;       // valid and good practice
    let _age = 30;       // valid variable
    let 1age = 30;       // Invalid variable

// Variables names can only contains letters, digits and an underscore character.
    let age1 = 30;       // Valid variable
    let age_num = 30;       // Valid variable
    let S@lary = 30;       // Invalid variable

// use underscore if we need to use two words as variable names.
    let first name = "jack"     // Invalid Variable
    let first_name = "jack"     // Valid Variable
    let firs- name = "jack"     // Invalid Variable

```

### Rust Constants : 

* Constants is a special type of variable whose value cannot be changed.

```rust
fn main(){
    // declare a float constant
    const PI: f32 = 3.14;
    print("value of PI  = {}", PI);
}
```

## Rust DataType : 

* We will have mainly four primary data type in rust : 
  * Integer
  * Floating-Point
  * Boolean
  * Character

### 1. Integer Type :

#### Signed Integer Type : 

```rust
    let number: i32 = 200;  // here we can use signed value like : positive & negative values.
    let num1: i32 = -200;
```
* `:` This colon acts as a separator, indicating that the type of the variable is about to be declared.
* `i32` This specifies that num will hold a 32-bit signed integer value.
* Here `i` : integer
* `32` is size of the integer

#### Unsigned Integer Type :

```rust
    let number: u32 = 200;
    let number: u32 = -200; // this will generate error.
```
* For unsigned integer it will take only positive value.
* Negative value here will generate error.

#### Categories of Integer Data Types in Rust : 


|   Size    |   Signed  |   Unsigned    |
|-----------|-----------|---------------|
|   8-bit   |    i8     |    u8         |
|   16-bit  |    i16    |    u16        |
|   32-bit  |    i32    |    u32        |
|   64-bit  |    i64    |    u64        |
|   128-bit |    i128   |    u128       |


### 2. Rust Floating Type : 

* Floating point type are used to store fraction (numbers with decimal pints).
* In rust Floating type can be divided into : 
  * `f32` : for 32-bit floating value.
  * `f64` : for 64-bit floating value.

```rust
    let x: f32 = 3.1
```

```Rust
fn main(){
    // f32 floating pint type
    let x: f32 = 3.1;

    // f54 floating point type
    let y: f63 = 43.000232;

    println!("X = {}", x);
    println!("Y = {}", y);
}
```
* Note : 
  * `f32` is used for single-precision floating type whereas `f64` is double-precision type. With double-precision `f64` can store data with larger decimal range and is considered more precision.

### 3. Rust Boolean Type : 

* In rust, a boolean data type have two possible values: `true` or `false`. For example,

```rust
fn main(){
// boolean value true
    let flag1: bool = true;
// boolean value false
    let flag2: bool = false;

    println!("flag 1 : {0}\nflag 2 : {1}", flag1, flag2);
}
```
### 4. Rust Character Type : 

* Character data type in Rust is used to store single character. For example, 

```rust
fn main(){
    // char type : 
    let value: char = 'z';

    println!("character = {}", value);
}
```
* the value will be store in single quotes.
* Here, `char` represent the character type and we use single quote to represent a character. 
* We can also store special character like `$`, `@`, `&`, etc. using the character type. 

* Note : we can also store the number as character using single quotes. for example : 
```rust
    let num_char: char = '5';
```

### Type Inference in Rust 

* So far we have mentioned the data type during the variable declaration. However, in Rust we can create variable without mentioning a datatype. For example : 

```rust
fn main(){
    let x = 51;
    println!("x = {}", x);
}
```
* In this case, Rust automatically identifies the data by looking at the value of the variable `x` and associate it with the variable. This process is known as `Type Inference`.

## Rust Type Casting 

* Type casting allows us to convert variable of one data type to another. In Rust, we use the `as` keyword to perform type casting. For example,

```rust
fn main(){

    // Create a floating-Point Variable
    let decimal: f64 = 23.2343;
    
    // converting floating point type to integer type 
    let integer = decimal as u16;
    
    println!("Decimal = {}", decimal);
    println!("Integer = {}", integer);
}
```

### Type conversion: Character to integer in Rust

```rust
fn main(){
    let character: char = 'A';
    // convert char type to u8;
    let integer = character as u8;

    
    println!("character = {}", character);
    println!("integer = {}", integer);

}
```
* Output : 
  
```plain
character = A
integer = 65
```

* Here we have convert `char` type to `u8` integer type, in output we have seen that the vale for `A` is `65`, this value is ASCII value.
* ASCII : American standards code for information interchange, Every character has some ASCII value.



## Rust Data Type : 

### Rust Array : 

* An array is list of elements of the same type. For example, if we want to store the first file natural number, we can create an array instead of creating five different variable.
* In rust array is created by using `[]` square brackets.

```rust
// array of natural number : 
let arr = [1, 2, 3, 4, 5];
```

#### Creating an array in rust : 

* In rust we can create an array in three different ways : 
  * Array with data type
  * Array without datatype
  * Array with default value

##### Array with data type : 

```rust
fn main(){
    // initialization of array with data type : 
    let numbers: [i32; 5] = [10, 20, 30, 40, 50];

    println!("array element are = {:?}", numbers);
}
```
* Here 
