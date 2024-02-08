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








## Rust Type Casting :

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





## Rust Loop :

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

* Tuple can hold different datatype. For example,

```rust
let tuple = ("hello", 5, 5.34);
```

* In Rust, we have used parenthesis or small bracket `( )` to create a tuple and it is able to store different types of value.

* **NOTE :** In Rust, tuples have a fixed size and cannot grow or shrink after they have been created.

### Creating a Tuple in Rust : 

* In Rust, we can create a tuple in two different ways:
    1. Tuple with datatype
    2. Tuple without datatype

#### 1. Tuple with datatype 

* While creating a tuple, we can mention the type of data it is storing. For example,
```rust
// create a tuple with datatype : 
let student_info: (&str, u8, f32) = ("Prince", 28, 9.99);
```
* Here, 
  * let `student_info: (&str, u8, f32)` - specifies the variable name and the data type of tuple elements.
  * `("Prince", 28, 9.99)` - specifies the elements of the tuple.

* Example : 
```rust 
fn main(){
    // initialization of tuple with datatype
    let tuple: (&str, u8, f32) = ("Prince", 28, 9.99);
    
    println!("Tuple Content = {:?}", tuple);
}
```
* Output : 

```plain
Tuple Content = ("Prince", 28, 9.99)
```

#### 2. Tuple without Data Type in Rust 

* We can define tuple without having consideration of datatype of data,
* Syntax

```rust
// create a tuple without datatype
let student_info = ("Prince", 28, 9.99);
```

* Example: tuple without datatype : 
```rust
fn main(){
    // initialization of tuple without datatype
    let tuple = ("Prince", 28, 9.99);
    
    println!("Tuple Content = {:?}", tuple);
}
```

### Accessing Elements in a Tuple : 

* Each element in a tuple is associated with a unique sequence of numbers. The number is known as the `tuple index` or just `index`.
* Suppose we have a tuple,
```rust
let random_tuple: (&str, u8, f32) = ("Prince", 23, 99.9);
```
* The tuple indexes of the random_tuple looks like,
  * `random_tuple.0` - access the element at index 0 (first element)
  * `random_tuple.1` - access the element at index 1 (second element)
  * `random_tuple.2` - access the element at index 2 (third element)

```rust
fn main(){
    let random_tuple: (&str, u8, f32) = ("Prince", 23, 99.9);

    // accessing first element at index 0 
    println!("Value at Index 0 = {}", random_tuple.0);

    // accessing second element at index 1 
    println!("Value at Index 1 = {}", random_tuple.1);

    // accessing third element at index 2
    println!("Value at Index 2 = {}", random_tuple.2);
}
```
* Output :
```plain
Value at Index 0 = Prince
Value at Index 1 = 23
Value at Index 2 = 99.9
```

* **Note :** The tuple index always starts at 0; hence the first element of the tuple is at position 0, not 1.


### Mutable Tuple 

* In Rust, a tuple is immutable, which means we cannot change its element once it is created.
* However, we can create a mutable array by using `mut` keyword before assigning it to a variable. For example,

```rust
// create a mutable tuple
let mut mountains = ("Prince", 23, 99.9);
```
* Let's take a look at an example,
```rust
fn main(){
    let mut random_tuple: (&str, u32, f32) = ("Prince", 23, 99.9);

    println!("Original tuple : {:?}", random_tuple);

    // changing hte value of tuple.
    random_tuple.0 = "Sahil";
    random_tuple.1 = 12204019;

    println!("Changed tuple : {:?}", random_tuple);
}
```
* Output : 
```plain
Original tuple : ("Prince", 23, 99.9)
Changed tuple : ("Sahil", 12204019, 99.9)
```

### Destructuring a Tuple :

* We can break down tuple into smaller variable in Rust, Known as Destructuring.
* Suppose we have a tuple,
```rust
let tuple = ("Ramesh jonas", 38, 172);
```
* Now we can perform Destructuring using,
```rust
let (name, age, height) = tuple;
```
* Now, we access the `name`, `age`, and `height` variable directly without using tuple indexes.
  * `name` name of `tuple.0`
  * `age` name of `tuple.1`
  * `height` name of `tuple.2`

* We can name variable as we like while Destructuring a tuple.

* **Note :** Destructuring a tuple is also known as **tuple unpacking**.

* Example : Destructuring a Tuple

```rust
fn main(){

    let tuple_name = ("Ramesh jonas", 38, 172);

    // Destructuring a Tuple
    let (name, age, height) = tuple_name;

    println!("name = {name}");
    println!("age = {age}");
    println!("height = {height}");
}
```
* Output : 

```plain
name = Ramesh jonas
age = 38
height = 172
```






## Rust Struct : 

* Rust structs or structures are user-defined data types used to store different types of data together.
* Suppose we want to store a person's name, age, and height. To do this, we can create variables for each property/field.

```rust
let personName: String = String::from("Sammy Kamkar");
let personAge: u8 = 38;
let personHeight: u8 = 178;
```
* This problem with this approach is we have to maintains all these variables separately. To store these fields for more than one person, we will have to create different variables for each person.

* Instead, we can create a struct to store all the fields together as a single unit. For example,

```rust
struct Person{
    name: String,
    age: u8,
    height u8
}
```
### Defining a Struct in Rust

* In Rust, we can use `struct` keyword to define a structure. The syntax of a structure is:
```rust
struct struct_name {
    field1: datatype,
    field2: datatype,
    field3: datatype
}
```
* **NOTE :** Here we are separating fields with `,` instead of semicolon.
* **NOTE :** We can define struct outside or inside `main()` function.

### Instantiating Rust Struct 

```rust
// Define a structure 
struct Person {
    name: String,
    age: u8,
    height: u8
}

fn main(){
    // Creating an instance of Person struct
    let person1 = Person{
        name: String::from("Prince"),
        age: 23,
        height: 190
    };
```

* Here, `Person {...}` creates an instance of the Person struct, and we have assigned it to the `person1` variable.
* Inside that variable we are assigning the values..


* **NOTE :** The struct definition is template, and the struct instance fill in that template with data.

### Accessing Fields of a Struct 

* We can use the struct instances along with the dot `.` notation to access values of fields in a structure. For example,

```rust
// Defining a Person struct
struct Person {
    name: String,
    age: u8,
    height: u8
}

fn main(){
    // instantiate person struct
    let person1 = Person{
        name: String::from("Prince"),
        age: 23,
        height: 190
    };

    // accessing the values of person1 struct
    println!("Person name = {}", person1.name);
    println!("Person age = {}", person1.age);
    println!("Person height = {}", person1.height);
}
```
* Or defining struct inside `main()` : 

```rust
fn main(){
    // Defining a Person struct
    struct Person {
        name: String,
        age: u8,
        height: u8
    }

    // instantiate Person struct : 
    let person1 = Person{
        name: String::from("Prince"),
        age: 23,
        height: 190
    };

    // accessing the values of person1 struct
    println!("Person name = {}", person1.name);
    println!("Person age = {}", person1.age);
    println!("Person height = {}", person1.height);
}
```

* Output will be same for these two different codes : 

```plain
Person name = Prince
Person age = 23
Person height = 190
```

### Destructuring Fields of a Rust Struct

* Destructuring is the process of breaking down field of a data type(array, tuple, etc.) into smaller variables. We can break down the struct into smaller variables in Rust.
* Suppose we have a struct and a struct instance,

```rust
struct Person {
    name: String,
    age: u8,
    height: u8
}

let person1 = Person{
    name: String::from("Prince"),
    age: 23,
    height: 190
};
```

* We can now perform destructuring using: 

```rust
let Person {name, age, height} = person1;
```
* We can destructuring using `let` keyword, where left side of `=` is struct, and right side is instance of struct.
* Now, we can access the `name`, `age`, and `height` field using the field names directly:
  * `name` instead of `person1.name`
  * `age` instead of `person1.age`
  * `height` instead of `person1.height`

* **Note :** The name of variable while destructuring should be the same as the name of the fields. 

#### Example : Destructuring Fields of Struct

```rust
fn main(){
    // Defining a Person struct
    struct Person {
        name: String,
        age: u8,
        height: u8
    }

    // instantiate Person struct : 
    let person1 = Person{
        name: String::from("Prince"),
        age: 23,
        height: 190
    };

    // destructure person struct into name, age, and height variables
    let Person {name, age, height} = person1;

    // accessing the values of person1 struct
    println!("Person name = {}", name);
    println!("Person age = {}", age);
    println!("Person height = {}", height);
}
```

* Output : 

```plain
Person name = Prince
Person age = 23
Person height = 190
```

* The pattern on the left has declarations, and the right side of the expression has a struct instance.








## Rust Function : 

* Function are reusable block of code, which used to perform a specific task. example, if we want to create a program to add two numbers, then we can create a Rust function to add to numbers. Now, we can reuse this same function whenever we add two numbers.

### Defining a Function in Rust 

* for defining a function we use `fn` keyword. The syntax of a function is, 

```rust
fn function_name(arguments){
    // code

}
```
* Example : 
```rust 
fn greet(){
    // code

}
```
* Here, 
  * `fn` - keyword used to create a function in Rust.
  * `greet()` - name of the function.
  * `// code` - function body
  * `{ }` - start and end of the function body.s

* Now let's complete the `greet()` function to print `hello world!`.

```rust
fn greet(){
    println!("hello World!");
}

fn main(){

}
```
* When we run the code, we will not get any output. This is because here we are just defining a function. To execute a function, we need to call it.

### Calling a Function : 

* We use the name of the function and parentheses `()` to call a function .
* Let's complete the above example now.

```rust
fn greet(){
    println!("Hello World!");
}

fn main(){
    // function call : 
    greet();
    
}
```
* Inside `main()`, we can call function.
* `main()` is special type of function, when we execute the program, it's the first function that executes.

* **Example :** Function to Add two numbers in Rust
```rust
// function to add two numbers in Rust
fn add(){
    let a = 5;
    let b = 10;
    let sum = a + b;
    println!("Sum of a and b = {}", sum);
}

fn main(){
    // function call 
    add();
}
```
* Output : 
```plain
Sum of a and b = 15
```

*** 
* **NOTE :** Rust code uses a small case as the convention for defining a function name. An extended function name with multiple words will have underscores in between words.

* The convention we are describing suggests using lowercase letters for function names and using underscores to separate words in longer function names. examples,

```rust
// Function name with a single word
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Function name with multiple words
fn calculate_total_price(item_price: f64, quantity: i32) -> f64 {
    item_price * quantity as f64
}
```
* Or we can use `#![allow(non_snake_case)]` on top our code to ignore the names & casing.
*** 

### Function parameters in Rust

* We can define our parameter inside the parenthesis `()`.

```rust
// function to add two numbers in Rust
fn add(a: i32, b: i32){
    let sum = a + b;
    println!("Sum of a and b = {}", sum);
}

fn main(){
    let a = 5;
    let b = 10;

    // function call 
    add(a,b);
}
```

* Here, during function definition we have passed the name of the variable and their datatype like : `a: i32`. Then the numbers of parameters, like in our case we have two, `a` and `b`.
* Also during calling we can pass the arguments to function call as function name then inside the parenthesis we can pass the value : `add(a,b)`.

### Function with Return value in Rust

* For making function to return something we provide `-> i32` before opening curly brackets `{`
* Inside the function body we use `return` keyword to return our result.
* Example : 

```rust
// function to add two numbers in Rust
fn add(a: i32, b: i32) -> i32 {
    let sum = a + b;
    return sum;
}

fn main(){
    let a = 5;
    let b = 10;
    
    // function call inside println
    println!("Sum of a and b = {}", add(a,b));
}
```
* Output : 

```plain
Sum of a and b = 15
```






## Rust Variable Scope : 

* In Computer programming, a variable's scope defines the region in which the variable is available for use.

```rust
fn main(){
    // this variable has scope inside the main function block
    let age = 31;
    ...
}
```
* Here, the `age` variable has scope inside the body `{...}` of the `main()` function,
* `NOTE :` Each variable in Rust has a scope that is valid inside a block. A block is a collection of statements enclosed by `{ }`.

### Working of variable scope in Rust

```rust
fn main(){
    // scope of outer_var variable is inside the main()
    let outer_var = 100;

    // start of inner code block
    {
        // scope of inner_var variable is only inside this new code block
        let inner_var = 200;
        println!("inner var = {}", inner_var);
    }
    // end of the inner code block

    println!("inner var = {}", inner_var);
    println!("outer var = {}", outer_var);

}
```

* Here, if we try to print the `inner_var` outside of the inner code block, the program fails to compile, and we encounter an error.

```plain
  --> src/main.rs:15:32
   |
15 |     println!("inner var = {}", inner_var);
   |                                ^^^^^^^^^ help: a local variable with a similar name exists: `outer_var`
```
* The rust compiler could not find `inner_var` in scope as we tried to print the variable outside the inner code block.
* To fix this, we can do the following,

```rust
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
```
* Output : 

```plain
inner var = 92
outer var = 32
outer var = 32
```

### Variable Shadowing in Rust


* In Rust, when a variable declared within a particular scope has same name as a variable declared in outer scope, it is known as **variable shadowing**.
* We can use same variable name in different scope block in the same program. Example,

```rust
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
```
* Output : 

```plain
random variable before shadowing in inner block = 100
random after shadowing in inner block = abc
random after shadowing in outer block = 100
```
* The `random` variable inside the inner block will shadow the value of the outer block so that the inner block will have the `"abc"` value. However, the value of the `random` variable remains the same outside of the inner block.

### Variable Freezing in Rust 

* We can freeze a variable in Rust by using shadowing and immutability. Once a variable is frozen, we cannot change the variable value in the inner scope. Example,

```rust
fn main(){
    
    let mut age = 100;

    {
        // shadowing by age variable
        let age = age;

        age = 2;

        println!("age variable inner block = {}", age);
    }
    // end of the inner block

    // age variable is not frozen in outer block
    age = 3;

    println!("age variable outer block = {}", age);
}
```

* Output as an error : 

```plain 
error[E0384]: cannot assign twice to immutable variable `age`
 --> src/main.rs:9:9
  |
7 |         let age = age;
  |             ---
  |             |
  |             first assignment to `age`
  |             help: consider making this binding mutable: `mut age`
8 |
9 |         age = 2;
  |         ^^^^^^^ cannot assign twice to immutable variable
```

* In  the above example, we have assigned the mutable variable of the outer block named age to the same immutable variable in the inner scope.

```rust
fn main(){
    let mut age = 100;
    {
        let age = age;
        ...
    }
    ...
}
```

* In this above example, if we are shadowing the `mutable variable` with an `immutable variable`, and when we try to change the value of that variable within the scope, our variable get's frozen.

* Fixed code of above example :

```rust
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
```
* Output : 

```plain
age variable inner block = 100
age variable outer block = 3
```

* **NOTE :**  When we are shadowing any outer scope mutable variable inside the inner scope as immutable variable, & if we are trying to change the value of inside scope variable, then it get's Freeze inside the inner scope, but in outside scope it can work as mutable(means we can change the value outside of the scope).





## Rust Closure 

* In Rust, closures are `function without names`. They ar also known as `anonymous function` or `lambdas`.

### Defining a Closure in Rust

* We can define closure with help of `||` i.e start of closure, & after we can specify our statements.
 

```rust
fn main(){

    // defining a closure to print a text
    let plain_text = || println!("defining closure");

    // call the closure
    plain_text();
}
```
* Once a closure is defined, we need to call it just like calling a function. To call a closure, we use the variable name to which the closure is assigned.

### Rust Closure with Parameters 

* In Rust, we can also pass parameters to a closure. For example,

```rust
// defining closure to add 1 to an integer
let add_one = |x: i32| x + 1;
```
* Here, Inside the `|...|`, we have defined our parameters, and after we will define our statements.
* `Note :` if we create closure with parameters, we need to also pass the value while calling the closure.

```rust
// call the closure with value 2
add_one(2);
```
* **Example : Rust closure with parameter**

```rust
fn main(){

    // define a closure add store it in a variable
    let add_one = |x: i32| x + 1;

    // call closure and store the result in a variable
    let res = add_one(2);

    println!("Result = {}", res);
}
```
## Multi-line Closure in Rust : 

* If our closure have multiple line statement, then we can use curly brackets `{...};` to specify our multi line statements.

```rust
#[allow(non_camel_case_types)]

fn main(){

    // define a closure add store it in a variable
    let reverse_of_Num = |x: i32| {
    let mut temp = x;
    let mut res = 0;
        while temp > 0 {
            let ld = temp % 10;
            res = res*10 + ld;
            temp = temp / 10;
        }
        println!("Rev is = {}", res);
    };
    // Calling 
    reverse_of_Num(123);
}
```
* Output : 
```plain
Rev is = 321
```



### Rust Closure with return type :

* During Definition of closure we can use return type to get the value from closure. Example,

```rust
fn main(){

    // define a closure add store it in a variable
    let add_one = |x: i32, y: i32| -> i32 {
        let res = x + y;
        println!("Adding these numbers!");
        res
    };

    // call closure and store the result in a variable
    let res = add_one(2, 3);

    println!("Result = {}", res);
}
```

* **NOTE :** In Rust, The last expression in the block of code implicitly become return value of that block, So we don't need to use the `return` keyword explicitly. Additionally when there is no semicolon at end of an expression it is treated as a return value.

* Example : Code with no return value & have multiple statement.

```rust
fn main(){
    // define a closure add store it in a variable
    let sum = |x: i32, y: i32| {
        let res = x + y;
        println!("Adding these numbers! {}",res);   // like it's not returning anything, so we use semicolon at end.
    };

    // calling closure
    sum(2, 3);
}
```

* Example : Code with return value & have multiple statements.

```rust
fn main(){

    // define a closure add store it in a variable
    let sum = |x: i32, y: i32| -> i32{
        let res = x + y; 
        println!("execution flow inside the closure! ");
        res // return res, this is why we didn't specify the semicolon at end.
    };

    // call closure and store the result in a variable
    let res = sum(2, 3);

    println!("Result = {}", res);
}
```
* **NOTE :** We can also explicitly use `return` keyword in closure statements. example,

```rust
fn main(){

    // define a closure add store it in a variable
    let reverse_of_num = |x: i32| -> i32 {
    let mut temp = x;
    let mut res = 0;
        while temp > 0 {
            let ld = temp % 10;
            res = res*10 + ld;
            temp = temp / 10;
        };
        return res; // using return keyword & semicolon. or if we don't use semicolon it will work fine.
    };
    // Calling 
    println!("Rev is = {}", reverse_of_num(123));
}
```
* This will generate output : `Rev is = 321`

### Closure Environment Capturing in Rust

* Closure has a unique feature that allows it to allows it to capture the environment. This means the closure can use the values in its scope. For example,

```rust
fn main(){
    let num = 100;

    // Closure that capture the num variable
    let print_num = || println!("Number = {}", num);

    // closure calling 
    print_num();
}
```

* Here, the closure bound to `print_num` uses the variable `num` which was not defined in it. This is known as closure environment capturing.
  

### Closure Environment Capturing Modes in Rust 

* Environment capturing closure can be of 3 different modes based on the variable and the closure definition.

1. Variable is not modified inside closure
2. Variable is modified inside closure
3. variable is moved inside closure

#### 1. Variable is not modified inside closure :



#### 2. Variable is modified inside closure :



#### 3. Variable is moved inside closure :




### Why to use Closure : 

* `Conciseness`: Closures can often provide more concise syntax compared to defining a separate named function.

* `Flexibility`: Closures can capture variables from their surrounding scope, making them very flexible for use in various contexts.

* `Readability`: In some cases, using a closure inline can improve code readability by keeping related functionality close together.

* `Callbacks and iterators`: Closures are commonly used in Rust for tasks like callbacks, iterators, and event handling, where passing behavior as an argument is necessary.

* In summary, while functions are more suitable for reusable and well-defined pieces of code, closures offer flexibility, conciseness, and the ability to capture variables from their enclosing scope, making them useful for certain types of tasks, especially when writing more functional-style code.