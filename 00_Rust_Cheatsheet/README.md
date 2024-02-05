# Rust General Concepts : 

## Printing Output : 
* For printing text we can use `print!()` or `println!()` macro.

```Rust
fn main(){
    println!("This is print statement!");

}
```
### Print variable : 

* for printing variable we will be using `{}` curly braces to fetch the variables.

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

* With using `mut` Keyword during variable declaration we can make our variable as mutable.
  
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

### Error while Converting integer to character : 

* Rust allows `u8` integer size only.

```rust
fn main(){
    let integer: i32 = 65;

    // converting integer to char using as keyword
    let character = integer as char;

    println!("Integer = {}", integer);
    println!("Character = {}", character);
}
``` 

* Here we got an error in output :

```plain
error[E0604]: only `u8` can be cast as `char`, not `u32`
 --> src/main.rs:8:21
  |
8 |     let character = integer as char;
  |                     ^^^^^^^^^^^^^^^
  |                     |
  |                     invalid cast
  |                     help: try `char::from_u32` instead: `char::from_u32(integer)`

For more information about this error, try `rustc --explain E0604`.
error: could not compile `guessing_game` (bin "guessing_game") due to previous error
```
* If we go on `rustc --explain E0604` we will know that, char is unicode scalar value, Unicode scalar value are small integer number & fit in the range of `u8` `unsigned 8-bit`.
  
### Type Casting: Boolean to integer in Rust 

```rust
fn main(){
    let boolean1: bool = false;
    let boolean2: bool = true;

    // convert boolean type to integer
    let integer1 = boolean1 as i32;
    let integer2 = boolean2 as i32;
}
```

* Here `False` will convert to value `0`, & `True` will be `1`.

### Limitation of Type Casting : 

* There are limitations while performing type casting in Rust. Not all datatype are converted to one another.

* example : we can't covert `floating` to a `character`.

```rust
fn main(){
    let decimal: f32 = 65.2133;

    // convert float to char
    let character = decimal as char;
    
    println!("decimal = {}", decimal);
    println!("character = {}", character);
}

```
* output generate error : 

```plain
error[E0604]: only `u8` can be cast as `char`, not `f32`
 --> main.rs:5:19
  |
5 |   let character = decimal as char;
  |                   ^^^^^^^^^^^^^^^ invalid cast
```










## Rust Operators : 

* Operators are Symbols that performs operations on value or variable
* Rust programming provides various operators that can be categorized into the following major categories : 
  * Arithmetic Operator 
  * Assignment Operator
    * Compounding Assignment Operator 
  * Logical Operators 
  * Comparison Operators 

### 1. Arithmetic Operator in Rust : 


|       Operators    |  Example   |
|--------------------|------------|
|   `+` ( Addition ) |  `a + b`   |
|   `-` ( Subtraction ) |  `a - b`   |
|   `*` ( Multiplication ) |  `a * b`   |
|   `/` ( Division ) |  `a / b`   |
|   `%` ( Remainder ) |  `a % b`   |



### Assignment Operator : 

```rust
let mut x = 1;
```
#### Compound Assignment Operator : 


|       Operators    |  Example   | Equivalent To   |
|--------------------|------------|---------|
|   `+=` ( Addition assignment ) |  `a += b`   | `a = a + b` |
|   `-=` ( Subtraction assignment) |  `a -= b`   | `a = a - b` |
|   `*=` ( Multiplication assignment) |  `a *= b`   | `a = a * b` |
|   `/=` ( Division assignment) |  `a /= b`   | `a = a / b` |
|   `%=` ( Remainder assignment) |  `a %= b`   | `a = a % b` |

```rust
    let mut x = 1;
    // compound assignment operator
    x += 3;   // equivalent to x = x + 3
```

### Comparison Operator :

* We use Comparison operator to compare two values or variable. for example,

```rust
    6 > 5
```
* Here, the relation operator `>` (greater than) checks if `6` is grater than `5`. 
* A relation operator returns : 
  * `true` if the relation b/w two value is correct.
  * `false` if the relation is incorrect.

* `NOTE` : Relation operator is also known as Comparison operator.

| Operator | Example | Description |
|----------|---------|-------------|
| `>` (Greater than)    |   `a > b` |   `true` if `a` is grater than `b` | 
| `<` (Less than)    |   `a < b` |   `true` if `a` is less than `b` | 
| `>=` (Grater than or equal to)    |   `a >= b` |   `true` if `a` is grater or equal to `b` | 
| `<=` (Less than or equal to)    |   `a <= b` |   `true` if `a` is Less or equal to `b` | 
| `<=` (Less than or equal to)    |   `a <= b` |   `true` if `a` is Less or equal to `b` | 
| `==` (Equal to)    |   `a == b` |   `true` if `a` is equal to `b` | 
| `!=` (Not Equal)    |   `a != b` |   `true` if `a` is not equal to `b` | 

### Logic Operators :

* We use logical operators to perform logical decision decisions or operations. 
* Result will be in `true` or `false`, depending on the conditions. For example : 
```rust
(5 < 6) && (7 > 4)
```
* This will result `true`, as both the condition are true.
* There are mainly 3 logical operators in Rust.

| Operator  | Example   |   Description |
|-----------|-----------|---------------|
|   `&&` ( logical AND )    | `exp1 && exp2`    |   returns `true` if both `exp1` and `exp2` are `true` |
|   `\|\|` ( logical OR )    | `exp1 && exp2`    |   returns `true` if any one `exp1` or `exp2` is `true` |
|   `!` ( logical OR )    | `!exp`    |   returns `true` if expression is `false`, if it is `true` |

* `Note` : logical `AND` and `OR` operator are also called short-circuiting logical operators because these operators don't evaluate the whole expression in these case they don't need to. For example, in this expression
```rust
false || true || false
```
* The `||` operator evaluates to `true` because once the compiler sees a single `true` expression, it skips the evaluation and return `true` directly.










## Rust if..else : 

### Boolean Expression

* Before we learn about `if..else` expression, let's quickly understand boolean expression.
* Boolean expression is an expression which return the value either `true` or `false` as the output.

```rust
fn main(){
    let x = 7;

    // example of boolean expression
    let condition = x > 5;

    println!("{}", condition)
}
```
* output will be `true`.

### Rust if : 

```rust
    if condition{
        // block to execute
    }
```
* If the condition evaluates to : 
  * `true` - the code inside the `if` block is executed.
  * `false` - the code inside of the block is not executed.

```rust
fn main(){
    let number = 10;

    // condition to check if number is grater than zero
    if number > 0 {
        println!("{} is grater than 0", number);
    }
    println!("End of program");
}
```
* Here we don't use parenthesis like c, c++ or other language to specify condition inside it, although it will run, but cargo considered it as bad practice.
* however, when we write multiple & complex condition it's good to specify the condition in parenthesis, so that the readability is increased...

### Rust if..else Expressions

```rust
if condition {
    // execute when the condition is true
} else {
    // execute when the condition is false
}
```

### Rust if..else if Expressions 

* We can evaluate **multiple conditions** by combining `if` and `else` is an `else if` expression.

```rust
if condition1 {
    // code block 1
} else if condition2 {
    // code block 2
} else {
    // code block 3
}
```

*  if `condition1` is true then `condition2` and `code block 3` is skipped.
*  if `condition2` is true then `condition1` and `code block 3` is skipped.
*  if `condition1` and `condition2` are false, then  `code block 3` is executed.

### Nested if..else

* We an use `if..else` expression inside the body of other `if..else` expression. it is known as nested `if..else` in Rust. example,

```rust
fn main(){
    let number = -2;

    if number < 0  {
        if number  ==  -2 {
            println!("the current number is -2");
        } else {
            println!("the current number is {}", number);
        }
    } 
}
```





## Rust Loop 

* In rust we have three different keywords to execute a code block multiple times:
  * `loop`
  * `while`
  * `for`

### Rust Loop Expression

* In Rust, we use the `loop` expression to indefinitely execute a block of code. if we use a `loop`, the code execution inside of the loop code block doesn't stop and runs forever. example,

```rust
fn main(){
    // loop expression
    loop {
        println!("Loop forever!");
    }
}
```

* This example code will print **Loop forever!** indefinitely unless the user terminates the program. Since the loop runs forever, it is also known as an infinite loop.

#### Terminating loop in rust : 

* we can use some `counter` with some condition to break out from loop, or we can use `break` keyword to break the loop execution.

* **NOTE:** In Rust, we often user a loop `loop` and `break` together

* Example : Print first 10 natural number.

```rust 
fn main(){
    let mut num = 1;
    loop{
        if num == 11 {
            break;
        }
        println!("{}",num);
        num += 1;
    }
} 
```

### Rust while loop Expression 

* We use the `while` loop to execute a code block till the condition is `true`. The syntax for the `while` expression is : 

```rust
    while condition {
        // code block
    }
    // code block outside while loop
```

* Here, until the condition is `true`, loop will execute the block of code again and again.

* Example : printing table of 20

```rust
fn main(){
    let num = 20;
    let mut cnt = 1;
    while cnt <= 10 {
        println!("{} x {} = {}", num, cnt, num*cnt);
        cnt += 1;
    }
} 
```

#### Nested while loop 

* we can use loop inside loop, example

```rust
/*
// we have to print this pattern : 

* * * *
* * * *
* * * *
* * * *

*/

fn main(){
    let mut row = 4;
    let mut cols = 4;
    while row > 0 {
        while cols > 0 {
            print!("* ");
            cols -= 1;
        }
        println!("");
        row -= 1;
        cols = 4;   
    }
} 
```

### Rust for loop Expression 

* The `for` loop in Rust is used to iterate a range of numbers. The syntax of `for` loop is:

```rust
for variable in lower_bound_number..upper_bound_number {
    // code block
}
```
* Let's take a look at an example,

```rust
fn main(){
    for i in 1..10 {
        print!("{} ", i);
    }
}
```
* output : 
```text
1 2 3 4 5 6 7 8 9 
```

* Here, the `lower_bound_number` is included & `upper_bound_number` is excluded.

* **NOTE:** The for loop is also known as a `for-in` loop because of its syntax.

* Example : Sum of first 10 natural number : 

```rust
fn main(){
    let mut sum = 0;
    for i in 1..11 {
        sum += i;
    }
    println!("Sum is {sum}");
}
```







## Rust Break and Continue : 

* During loop execute a block of code multiple times. However sometimes we might need to alter the flow of a loop by terminating it execution or skipping an iteration.
* In such case we use the Rust `break` and `continue` to alter the normal execution of loops. For example,
    * `break` - terminates the loop.
    * `continue` - skips the current iteration of the loop and moves on to the next.

* Example : 

```rust
fn main(){

    let mut num = 0;
    loop {
        num += 1;
        if num == 5 {
            continue;
        }
        if num == 11{
            break;
        }
        print!("{num} ");
    }
}
```
* Output : 
```text
1 2 3 4 6 7 8 9 10
```







## Rust Array : 

* An array is list of elements of the same type. For example, if we want to store the first file natural number, we can create an array instead of creating five different variable.
* In rust array is created by using `[]` square brackets.

```rust
// array of natural number : 
let arr = [1, 2, 3, 4, 5];
```

### Creating an array in rust : 

* In rust we can create an array in three different ways : 
  * Array with data type
  * Array without datatype
  * Array with default value

#### Array with data type : 

```rust
fn main(){
    // initialization of array with data type : 
    let numbers: [i32; 5] = [10, 20, 30, 40, 50];

    println!("array element are = {:?}", numbers);
}
```
* Here `numbers` is name of the array
* `[i32; 5]` - `i32` is the predefined data type of array element & `5` is the size of the array.
* `[10, 20, 30, 40, 50]` are elements inside the array.

#### Array without datatype : 

```rust
fn main(){
    // initialization of array without data type
    let numbers = [1, 2, 3, 4, 5];

    println!("array of numbers = {:?}", numbers);
}
```
* Here we are defining the array without defining the datatype & size.

#### Array with Default value in Rust : 

```rust
fn main(){
    // initialization of array with default values 
    let number: [i32, 5] = [3, 5];
    println!("array of number = {:?}", number);
}
```
* Here `[i32; 5]` - represent the data type(`i32`), and size(`5`) of the array
* `[3; 5]` - is a repeat expression, here the value `3` will fill the array `5` times.

* `Note` : we can also omit the datatype and size while creating an array of default values. For example,

```rust
fn main(){
    // initialization array with default values :
    let numbers = [3; 5];

    println!("Array of numbers = {:?}", numbers);

}

```

##### Revision: Different ways to create array in rust 

```rust
fn main(){
    // array without datatype
    let a = [1, 2, 3, 4, 5];

    // array with datatype and size
    let b: [i32; 5] = [1, 2, 3, 4, 5];

    // array with default value;
    let c = [1; 5]; 
    let d: [i32; 5] = [1; 5];   // we can declare c with this way also

    println!("a = {:?}", a);
    println!("b = {:?}", b);
    println!("c = {:?}", c);
    println!("d = {:?}", d);
}
```
* Output : 

```text
a = [1, 2, 3, 4, 5]
b = [1, 2, 3, 4, 5]
c = [1, 1, 1, 1, 1]
d = [1, 1, 1, 1, 1]
```



### Access Elements of Rust Array : 

* In Rust index starts from `0`

```rust
fn main(){
    let color = ["red", "green", "blue"];
    // accessing value : 
    println!("1st color : {}",color[0]);
    println!("2nd color : {}",color[1]);
    println!("3rd color : {}",color[2]);

}
```
* We can access array element using indexing.



### Mutable Array in Rust : 

* In rust default variables values are immutable, which means we can't change their value once we declare, so it's same with array.
* However, we can create a mutable array by using `mut` keyword before assigning it to a variable. For example,

```rust
fn main(){
    // creating mutable array in rust : 
    let mut numbers: [i32; 5] = [1; 5];
    println!("Original value : {:?}", numbers);
    numbers[0] = 90;
    numbers[2] = 90;
    numbers[4] = 90;
    println!("After changed : {:?}", numbers);
    }
```
### Looping Through an Array in Rust :
* In Rust, we can use the `for..in` loop to iterate through an array. For example, 

```rust
fn main(){
    let color = ["red", "green", "blue"];

    // looping through an array to print it's index and value : 
    for index in 0..3{
        println!("Index: {} -- Value: {}", index, color[index]);
    }
}
```










## Rust Slice : 

* A rust slice is a data type used to access portion of data stored in collection like array, vector and strings.

* Suppose we have an array,

```rust
let number = [1, 2, 3, 4, 5];
```
*  Now, if we want to extract the 2nd and 3rd elements of this array. We can slice the array like this, 
```rust
let slice = &number[0..3];
```
* Here, let's look at the right-hand side of the expression,
  * `&numbers` - specifies a reference to the variable `numbers` (not the actual value)
  * `{1..3}` - is a notation for slicing the array from *start_index* `0`(inclusive) to *end_index* `3`(exclusive).

```rust

fn main(){
  let number = [1, 2, 3, 4, 5];
    println!("Original value : {:?}", number);
    let slice = &number[0..3];
    println!("Original value : {:?}", slice);
}
      
```
* Note : A slice is not the actual data like integer or floats, but a reference/ pointer  to the block. That's why we have used the `&` symbol before the variable name.

### Omit Indexes of Rust Slice : 

* While slicing a data collection, Rust allows us to omit either the start index or the end index or both from it's Syntax.

```rust
&variable[start_index..end_index]
```

#### 1. Omitting the stating index of a slice

```rust
fn main(){
    let numbers = [1,2,3,4,5];

    // omit the start index
    let slice = &numbers[..3];

    println!("array = {:?}", numbers);
    println!("slice = {:?}", slice);
}
```
* output : 

```text
array = [1,2,3,4,5]
slice = [1,2,3]
```

* if we use `&numbers[..3]`, it will start slicing from 0th index.

#### 2. Omitting the End index of a slice

```rust
fn main(){
    let numbers = [1, 2, 3, 4, 5];
    // omit the end index
    let slice = &numbers[2..];

    println!("array = {:?}", numbers);
    println!("slice = {:?}", slice);
}
```
* output : 

```plain
    array = [1,2,3,4,5]
    slice = [3,4,5]
```

* Here we are slicing it from starting index but we didn't specify the end index then it will slice an array till the last value.
* Note : the first value that we provided is excluded in output.

#### 3. Omitting both start and end index of slice

```rust
fn main(){
    let numbers = [1,2,3,4,5];
    
    // omit the start index and the end index 
    // reference the whole array
    let slice = &numbers[..]

    println!("array = {:?}", numbers);
    println!("slice = {:?}", slice);
}

```
* output

```text
array = [1,2,3,4,5]
slice = [1,2,3,4,5]
```

* Here, if we didn't specify the starting and ending index, then it will slice whole the array, or we can say it will get all the value.

* `**Note**`: if we want to slice in certain range, we can do with specifying the range, like [2..4], in array of size 5.





### Mutable Slice in Rust

* We can create a mutable slice by using the `&mut` keyword.
* Syntax will be : `let slice = &mut data_structure[start_index..end_index];`

```rust
let mut number = [1,2,3,4,5];
let slice = &mut number[1..4];
```
* Once the slice is marked as mutable, we can change value inside the slice. Let's see an example,

```rust
fn main(){

    let mut number = [1,2,3,4,5];
    
    println!("number = {:?}", number);

    let slice = &mut number[..4];
    
    println!("slice = {:?}", slice);
    
    slice[1] = 99;
    
    println!("changed slice = {:?}", slice);

}
```
* Output : 

```plain
number = [1, 2, 3, 4, 5]
slice = [1, 2, 3]
changed slice = [1, 99, 3]
```

* `**NOTE : **` attempting to create a mutable slice from the immutable array directly will result in a compilation error because Rust doesn't allow borrowing mutable references to immutable data. Example : 

```rust
fn main() {
    let data = [1, 2, 3, 4, 5];

    // This line will not compile
    let slice: &mut [i32] = &mut data;
    // Error: cannot borrow immutable local variable `data` as mutable
}
```







## Rust Tuple : 



