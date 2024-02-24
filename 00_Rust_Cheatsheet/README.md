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






## Rust Enum : 

* Enums (or enumeration) is a user-defined data type that allows us to select a value from a list of related values.

* In Rust, we use the `enum` keyword to create an enum. For example,

```rust
enum Sport {
    Basketball,
    Volleyball,
    Football,
    Cricket,
}
```
* Here, we have created an enum named `Sport` with a list of values `Basketball`, `Volleyball`, `Football` and `Cricket`.
* These enum value are known as `variants`.



*** 

* **Struct vs Enums**

* Structs (Structures):
  * Use structs when you need to represent a single entity with multiple properties.
  * A struct is a way to group multiple variables (fields) under a single name.
  * Each field within a struct can have a different data type.
  * Structs are often used to create records or objects that represent a single, cohesive unit of data.
  * Example:
```rust
struct Point {
    x: i32,
    y: i32,
}

let my_point = Point { x: 10, y: 20 };
```

* Enums (Enumerations):
  * Enums are suitable when you have a type that can exist in multiple forms or states.
  * An enum is a type that represents a set of named values, called variants.
  * Enums are useful when you have a finite set of possibilities, and each possibility is distinct.
  * Enums can have associated data with each variant, allowing for more complex data representations.
  * Example:

```rust
enum Shape {
    Circle(f64),
    Rectangle(i32, i32),
    Triangle(i32, i32, i32),
}

let my_shape = Shape::Circle(5.0);
```
* In this example, the Shape enum has three variants (Circle, Rectangle, and Triangle), each carrying different data types.
  
In summary, structs are used for defining structures with named fields, while enums are used for defining types that can have a finite set of distinct values (variants), often with associated data. Each has its own use case, and the choice between them depends on the nature of the data you are modeling.

***




### When to use Enum in Rust :

* Let's look at a situation where we want to define an enum.

* Suppose you are creating a program where you have to store a list of directions, and we know that there will be only four possible values for direction: North, East, West, and South.
* In this case, we can use an enum where each variant will be the direction.

```rust
enum Direction {
   North,
   East,
   South,
   West,
}
```
* Now we can access each of the enum variants whenever we have to use a direction in our program.

> **NOTE :** By Convention, we use the Pascal case for enum names and values. In Pascal case, we write the first letter of the word(s) in uppercase.


### Accessing Enum Variants in Rust : 

* To access enum values, we first have to create enum instances. 

```rust
enum Direction {
   North,
   East,
   South,
   West,
}
```

* Now, let's create instances of each enum variant:
```rust
  let north = Direction::North;
   let east = Direction::East;
   let south = Direction::South;
   let west = Direction::West;
```

* Here, `Direction::North` represent the enum variant `North` of the `Direction` enum, and we are assigning it to the variable `north`, Similarly we are doing for others.

### Rust Enum Data Type 
* Now let's see one example of how enum works in Rust.

```rust
// define enum Direction
#[derive(Debug)]
enum Direction {
   North,
   East,
   South,
   West,
}

fn main(){

   // Create instances of each enum variant:
   let north = Direction::North;
   let east = Direction::East;
   let south = Direction::South;
   let west = Direction::West;

    // print enum values
   println!("{:?}",north);
   println!("{:?}",east);
   println!("{:?}",south);
   println!("{:?}",west);
}
```
* Output : 

```plain
North
East
South
West
```

> **NOTE :** We can define enum inside or outside the main function.
> We have used `#[derive(Debug)]` above the enum definition, it's because this allows Rust to print the variants inside the enum.



### Initializing Enum Variants with Values in Rust 

* In Rust, we can also initialize enum variants by assigning individual values. For example,

```rust
#[derive(Debug)]
enum Result {
   Score(f64),
   Valid(bool),
}

fn main(){

   // Create instances of each enum variant:
   let num = Result::Score(3.14);
   let is_passed = Result::Valid(true);

   println!("num = {:?}",num);
   println!("isPassed = {:?}",is_passed);
}
```
* Output : 

```plain
num = Score(3.14)
isPassed = Valid(true)
```
* In the above example, we have created enum with different type of variants.
* Variants : 
  * `Score`- f64
  * `Valid` - boolean type


### Enum with different Data Types in Rust :

* In Rust, we can also create enums where the enums variants are of different data types : struct, tuple, string, etc.

1. **Enums with a struct variant**

```rust
enum Game {
    Quit,
    Position{ x : i32, y : i32},
}
```
* Here the `Game` enum has an anonymous struct `Position {x : i32, y: i32}` as a variants.

2. **Enum with a tuple variant**

```rust
enum Game {
    Quit,
    ChangeBackground(i32, i32, i32),
}
```
* Here, the `Game` enum has a tuple `ChangeBackground(i32, i32, i32)` as a variant.

3. **Enum with a string variant**

```rust
enum Game {
    Quit, 
    Print(String),
}
```
* Here, the `Game` enum has a `Print(String)` variant.

### Example: Enum Variants of Different Data Types

* Earlier, we see that enum variants can be of different data types. We can even create an enum where each variant is of different type. That is one variant will be a string, another one will be a struct and so on.. For example,

```rust
// define enum with multiple variants and data types
#[derive(Debug)]
enum Game {
   Quit,
   Print(String),
   Position { x : i32, y : i32},
   ChangeBackground(i32, i32, i32),
}

fn main(){

   // instances enum with variant:
   let quit = Game::Quit;
   let print = Game::Print(String::from("Sahilwep"));
   let position = Game::Position{x: 10, y: 20};
   let color = Game::ChangeBackground(200, 255, 255);

   // print enum values 
   println!("quit = {:?}", quit);
   println!("print = {:?}", print);
   println!("position = {:?}", position);
   println!("color = {:?}", color);
}
```

* Output : 

```plain
quit = Quit
print = Print("Sahilwep")
position = Position { x: 10, y: 20 }
color = ChangeBackground(200, 255, 255)
```

### Mutable Enum in Rust : 

* To create mutable enum we can use `mut` keyword while initializing the enum. Example,

```rust

fn main(){
   // define enum with multiple variants and data types
   #[derive(Debug)]
   enum Animal {
      Dog(String, f64),
      Cat(String, f64),
   }

   // initialize a mutable enum variant with values
   let mut dog = Animal::Dog(String::from("Benny"), 37.5);

   
   // initialize a non-mutable enum variant with values
   let cat = Animal::Dog(String::from("maya"), 22.4);

   // print enum values before changing : 
   println!("Dog before = {:?}", dog);
   println!("Cat before = {:?}", cat);

   // change the value of mutable enum variant
   dog = Animal::Dog(String::from("Sterling"), 21.3);

   // print enum values after change.
   println!("\n\nDog after = {:?}", dog);
   println!("Cat after = {:?}", cat);
}
```

* Output : 

```plain
Dog before = Dog("Benny", 37.5)
Cat before = Dog("maya", 22.4)


Dog after = Dog("Sterling", 21.3)
Cat after = Dog("maya", 22.4)
```
* Here, we have initialized an enum variant and assigned it to a variable `dog` using the `mut` keyword. This allows us to change the values of `dog` variable.












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
* 

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
  

#### Closure Environment Capturing Modes in Rust 

* Environment capturing closure can be of 3 different modes based on the variable and the closure definition.

1. Variable is not modified inside closure.
2. Variable is modified inside closure.
3. Variable is moved inside closure.

##### 1. Variable is not modified inside closure.

```rust
fn main(){
    let word  = String::from("Hello");

    // immutable closure
    let print_str = || {
        println!("word = {}", word);
    };

    // immutable borrow is possible outside the closure
    println!("length of word = {}", word.len());

    print_str();
}
```
* Outside : 
```plain
length of word = 5
word = Hello
```

* Here, the variable `word` is not modified inside the closure `print_str`. As the variable is immutable by default, we can make any number of immutable reference of `word` inside the closure. Notice that the closure variable `print_str` is also immutable.

* **NOTE :** This mode of capture is also known as `Capture by Immutable Borrow`.

##### 2. Variable is modified inside closure.

```rust
fn main(){
    let mut word  = String::from("Hello");

    // immutable closure
    let mut print_str = || {
        word.push_str(" World!");   // changing the value of word variable.
        println!("word inside = {}", word);
    };

    // calling closure
    print_str();
    
    // outside the word variable value is also changed!
    println!("word outside = {}", word );
}
```
* Output : 

```plain
word inside = Hello World!
word outside = Hello World!
```

* Here, the variable `word` is modified inside the closure `print_str` with `word.push("World!")`. Thus, we have to make the variable `word` mutable as well the closure variable `print_str`. This means no other references of the `word` variable can exist unless the closure is used.

* This mode of capture is also known as **Capture by Mutable Borrow**.

##### 3. Variable is moved inside closure

```rust
fn main(){
    let word  = String::from("Hello");

    // immutable closure
    let print_str = || {
        // variable is moved to a new variable.
        let new_word = word;
        println!("word inside = {}", new_word);
    };

    // calling closure
    print_str();
    
    // cannot immutable borrow because word variable has moved inside closure
    // println!("word outside = {}", word );
}
```
* Output : 
```plain
word inside = Hello
```
* Here, we have moved the variable `word` to a new variable `new_word` inside the closure. As the variable is moved, we cannot use it anywhere else except for inside the closure.

* This mode of capture is also known as **Capture by Move**.

* we can also move our variable inside the closure by using `move` keyword. example,

```rust
fn main() {
    let word = String::from("Hello");

    // Immutable closure with `move` keyword
    let print_str = move || {
        println!("word inside = {}", word);
    };

    // Calling the closure
    print_str();

    // Attempting to use `word` outside of the closure will result in a compilation error
    // println!("word outside = {}", word); // This line will cause a compilation error
}
```
* Here, we have used `move` keyword to move our variable `word`, & if we use this variable after the closure we can't access it.

* In Rust, the move keyword is used to convert any variables captured by reference or mutable reference to variables captured by value. It is used to transfer the ownership of a variable to the closure

*** 
* **CASE :** if we use this `move` closure, still we can access the value after that.

```rust
fn main() {
    let x = 10; // Variable to capture

    // Closure with `move` (owns `x`)
    let closure_with_move = move || println!("x is: {}", x); // Can be called later, even after `x` is gone

    closure_with_move();

    println!("access value after closure with move = {}", x); 
}
```

* `Solution :` this because the integer types implementation `Copy`
if a type imples Copy then it can be copied around freely instead of being unavailable after a move.
* If we try it with something that's not `Copy`, like a `String` or a `Vec<T>` instead, and you'll see it doesn't compile
?eval

```rust
let x = vec![10, 20, 30];

let closure_with_move = move || println!("x is {:?}", x);

closure_with_move();

println!("access value after move = {:?}", x); // error
```

*** 

##### Here's an example demonstrating each mode:

```rust
fn main() {
    let x = 42;
    
    // closure - moved
    let closure_once = move || {
        println!("x: {}", x);
    };

    // closure - modified
    let mut y = 10;
    let mut closure_mut = || {
        y += 1;
        println!("y: {}", y);
    };

    // closure - not_modified
    let closure_imm = || {
        println!("x: {}", x);
    };

    // Calling closures
    closure_once(); // This moves x into the closure, can't use x anymore
    closure_mut();  // This borrows y mutably
    closure_imm();  // This borrows x immutably
}
```







## Rust Stack and Heap : 

* Stack and Heap are parts of memory available to the Rust code to use at runtime.
* Rust is memory-safe programming language. To ensure that rust is memory-safe, it introduce concept like ownership, references and borrowing.
* To understand these concepts, we first understand how to allocate and deallocate memory in the Stack and Heap.

### The Stack 

* The Stack can be through of as stack of book. When we add more books, we add them on the top of the pile. When we need to a book, we take one from the top.
* The Stack insert value in order. It gets them and removes the values in the opposite order.
  * Adding data is called **pushing onto the stack**
  * Removing data is called **popping off the stack**
* Data Stored on the stack must have a fixed size during compile time. Rust by default, allocate memory on the stack for primitive types. Example,

```rust
fn foo () {
    let x = 999;
    let y = 333;
}
fn main() {
    let x = 100;

    foo();
}
```
* On stack first the `100` value will be inserted, then `999` then `333`.

* Rust automatically does allocation and deallocation of memory in and out of the stack.

### The Heap 

* As opposite to the stack, most of the time, we need to pass variable (memory) to different function and keep them alive for longer than a single function's execution. This is when we can use the `heap`.
* We can allocate memory on the heap using the `Box<T>` type. For example,

```rust
fn main(){
    let x = Box::new(100);
    let y = 222;

    println!("x = {}, y = {}" , x , y);
}
```
* On memory first address on stack reserved for later allocation of memory address, let say `0` has value `???`, then the `222` will be allocated.
* The pointer for first allocation will be pointed to heap memory let say `2342` with value `100` & this address will point to the `0` on stack.


### Difference between Stack and Heap

| `Stack` | `Heap` |
|---------|--------|
| Accessing data in the stack is faster.  | Accessing data in a heap is slower. |
| Managing memory in the stack is predictable and trivial. | Managing memory for the heap (arbitrary size) is non-trivial. |
| Rust stack allocates by default. | Box is used to allocate to the heap. |
| Primitive types and local variables of a function are allocated on the stack. | Data types that are dynamic in size, such as `String`, `Vector`, `Box`, etc., are allocated on the heap. |






## Rust Vector :

* Vector is dynamic (resizable) data structure that can store a list of element of the same type. Being a resizable data structure, vectors can grow and shrink at runtime.

### Create a Vector in Rust

* In Rust, we can create a vector using the `vec!` macro. For example,

```rust
let v = vec![1, 2, 3];
```
* Here, we are creating a vector using the `vec!` macro with some initial values.
  * `let v` - the name of the variable
  * `vec![1, 2, 3]` - initialize a vector with integer value **1, 2, 3**.

* By looking at the type of the values provided to the macro, Rust will automatically set the vector type. For example, the vector type of the above vector is `Vec<i32>`.

* We can also define the vector type overselves using the `vec!` macro
```rust
let v: Vec<u8> = vec![1, 2, 3];
```
* **Example:** Creating a vector in Rust

```rust
fn main(){
    // Vector Deceleration without datatype
    let v = vec![1, 2, 3];
    // Vector Deceleration with datatype
    let v1: Vec<u8> = vec![4, 5, 6];


    println!("vector is : {:?}", v);
    println!("vector is : {:?}", v1);
}
```
* Output : 

```plain
vector is : [1, 2, 3]
vector is : [4, 5, 6]
```

### Accessing Elements of a Vector 

* We can access each element of a vector using `index` i.e start from `0`.

```rust
fn main(){
    // Vector Declaration
    let v = vec![1, 2, 3];

    // accessing element of vector 
    println!("vector is : {}", v[0]);
    println!("vector is : {}", v[1]);
    println!("vector is : {}", v[2]);
}
```
#### Accessing elements of a vector using the get() method

* We can also access the element of the vector with the `get()` method and the index of the element.

```rust
fn main(){
    // Creation of vector
    let v = vec![1, 2, 3];

    println!("vector is : {:?}", v.get(0));
    println!("vector is : {:?}", v.get(1));
    println!("vector is : {:?}", v.get(2));
}
```
* As we can see, the output returns a value `Some(1), Some(2), Some(3)` of the `v<T>` Type.

* The advantages of using `get()` method is over just using the vector index to access the element directly is that it will not error if the vector index is out of range.

### Adding value to a Vector 

* We can add value to a vector by creating a `mutable` vector in rust.
* We can use `push()` method to push values in vector.

```rust
fn main(){
    // creating mutable vector, so that we can insert the value inside the vector.
    let mut v: Vec<u32> = vec![29, 23, 7, 23];

    // inserting value at the last to the vector
    v.push(12);
    v.push(92);

    println!("vec = {:?}", v);
}
```
* Output : 
```plain
vec = [29, 23, 7, 23, 12, 92]
```
* `NOTE :` here the values are inserted at the last of the vector.

### Removing value from a vector : 

* We can remove values from a vector by making mutable and with the `remove()` method. 
* We need to pass the index inside the `remove()` method.

```rust
fn main(){
    // creating of mutable vector 
    let mut v: Vec<u32> = vec![29, 23, 7, 23];

    // inserting value inside the vector
    v.push(12);
    v.push(92);

    println!("Original Vec = {:?}", v);
    
    // removing value from vector by passing index.
    v.remove(2);

    println!("Original Vec = {:?}", v);
}
```
* Output : 

```plain
Original Vec = [29, 23, 7, 23, 12, 92]
Original Vec = [29, 23, 23, 12, 92]
```
* Here, we have passed the index `2` which has value `7`, & it get removed.

* Output : 

```plain
Original Vec = [29, 23, 7, 23, 12, 92]
Original Vec = [29, 23, 23, 12, 92]
```
   
* `NOTE :` Removing an element will shift all other values in the vector by one **(-1 index)**.


### Looping through a Vector in Rust 

* We can use the `for..in` or `while` or `loop` to iterate through a vector. For example,

```rust
fn main(){
    // creating of mutable vector 
    let mut v: Vec<u32> = vec![29, 23, 7, 23];

    // inserting value inside vector
    v.push(12);
    v.push(92);
    v.push(83);
    v.push(32);

    // looping through a vector to print it's index and value : 
    for index in 0..8 {
        println!("Index = {}, Value = {}", index, v[index]);
    }
}
```

* Output :

```plain
Index = 0, Value = 29
Index = 1, Value = 23
Index = 2, Value = 7
Index = 3, Value = 23
Index = 4, Value = 12
Index = 5, Value = 92
Index = 6, Value = 83
Index = 7, Value = 32
```

### Creating a Vector using `Vec::new()` Method 

* `Vec::new()` : it creates new constructor of vector

```rust
let v: Vec<u8> = Vec::new();
```
* `Vec::new()` - initialize an empty vector with the `new()` method
* Example : 
```rust
fn main(){
    // Creating mutable vector by calling constructor of vector
    let mut v: Vec<u8> = Vec::new();

    // insertion of values inside the vector.s
    v.push(10);
    v.push(12);
    v.push(13);
    v.push(14);

    for i in 0..4 {
        println!("Value is : {}", v[i]);
    }

}
```
* Output : 

```plain
Value is : 10
Value is : 12
Value is : 13
Value is : 14
```

* Resource : [Multiple Methods for Vector Creation](https://www.linkedin.com/pulse/vector-creation-iteration-rust-amit-nadiger/)











## Rust String 

* String in Rust is a sequence of  **Unicode characters encoded in UTF-8**.



### Creation of String

```rust
fn main(){

    // Creation of immutable string :
    let s = "Sahil";
    // another method :
    let name = String::from("Prince");

    println!("{}", s);
    println!("{}", name);
}
```
* Here we have created string using two method, we can directly pass the value inside the `"..."` double quotes, Cargo will determine it as string.
* Or we can use `String::From("....")` method.
* Strings are of immutable type.

### Mutable String in Rust

* We can use `mut` keyword before assigning a string to a variable.

```rust
fn main(){
    // Creation of mutable string :
    let mut name = String::from("Sahil");
    println!("{}", name);

    // after pushing value inside string
    name.push_str(" Sharma");

    println!("{}", name);}
```

### String Slicing in Rust

* We can slice a string in Rust to reference a part of the string.

```rust
fn main(){
    let word = String::from("Hello World!");

    // slicing a string 
    let slice = &word[0..5];

    println!("string = {}", word);
    println!("slice = {}", slice);
}
```
* Here, `&word[0..5]` is a notation for slicing the string stored in variable `word` from start index **0** (inclusive) to end index **5** (exclusive).
* The `&` (ampersand) in the slicing syntax signifies that it is a string reference. it is not actual data.

### Iterating over String

* We can use the `chars()` method of the string type to iterate over a string. For example,

```rust
fn main(){
    let word = String::from("Hello");

    // iterating through each character in string using chars() method
    for char in word.chars() {
        println!("{}", char);
    }
}
```

### Creating an Empty String with `String::new()`

* We can create an empty string using the `String::new()` method. For example,

```rust
fn main(){
    let mut word = String::new();

    println!("Original String  = {}", word);
    
    // append a string to the word variable
    word.push_str("Hello World!");

    println!("Changed String  = {}", word);
}
```
* Output : 
```plain
Original String  = 
Changed String  = Hello World!
```












## Rust HashMap : 

* The Rust HashMap data structure allows us to store data in **key-value pairs**. Here are some of the features of hashmap:
  * Each value is associated with a corresponding key.
  * Keys are unique, whereas value can duplicate.
  * Values can be accessed using their Corresponding keys.

### Creating a HashMap in Rust 

* HashMap is part of the Rust standard collections library, so we must include the `HashMap` module in our program to use it.
* We can import the hashmap by using this deceleration.

```rust
use std:collections::HashMap;
```

* Now, we can create a hashmap using the `new()` method in the `HashMap` module. For example,

```rust
let mut info: HashMap<i32, String> = HashMap::new();
```
* `let mut info` - declare a mutable variable info.
* `HashMap<i32, String>` - type of the HashMap where the key is a integer and value is a String.
* `HashMap::new()` - creates a new HashMap

* Example Creating a HashMap : 

```rust
use std::collections::HashMap;

fn main(){
    // create a new HashMap
    let mut info: HashMap<i32, String> = HashMap::new();

    println!("HashMap = {:?}", info);
}
```
### HashMap Operations 
* The `HashMap` module provides various method to perform basic operations in a hashmap.
  * Add Element
  * Access Value
  * Remove Element
  * Change Element

#### Add Element to a HashMap in Rust

* We can use `insert()` to add an element(key-value pairs) to a hashmap.
  

```rust
use std::collections::HashMap;

fn main(){
    // create a new HashMap
    let mut fruits: HashMap<i32, String> = HashMap::new();

    // add key-value in a hashmap
    fruits.insert(1, String::from("Apple"));
    fruits.insert(2, String::from("orange"));

    println!("Fruits = {:?}", fruits);
}
```

* Output : 
```plain
Fruits = {1: "Apple", 2: "orange"}
```

#### Access Values in a HashMap in Rust

* We can use `get()` to access a value from the given hashmap.

```rust
use std::collections::HashMap;

fn main(){
    // create a new HashMap
    let mut fruits: HashMap<i32, String> = HashMap::new();

    // add key-value in a hashmap
    fruits.insert(1, String::from("Apple"));
    fruits.insert(2, String::from("orange"));

    // access values in the hashmap
    let first_fruit = fruits.get(&1);
    let second_fruit = fruits.get(&2);
    let third_fruit = fruits.get(&3);

    println!("First Fruits = {:?}", first_fruit);
    println!("Second Fruits = {:?}", second_fruit);
    println!("Third Fruits = {:?}", third_fruit);
}
```
* Output : 

```plain
First Fruits = Some("Apple")
Second Fruits = Some("orange")
Third Fruits = None
```
* Here, we have used `&` ampersand and the key(`&1`, `&2`) as an argument to the `get()` method.
* The `third_fruit` return `None` value because `&3` doesn't match anything that's in the hashmap.

#### Remove Element from a HashMap in Rust

* We can remove elements from a hashmap by providing a key to the `remove()` method. For Example,

```rust
use std::collections::HashMap;

fn main(){
    // create a new HashMap
    let mut fruits: HashMap<i32, String> = HashMap::new();

    // add key-value in a hashmap
    fruits.insert(1, String::from("Apple"));
    fruits.insert(2, String::from("orange"));

    println!("First Fruits Before Removing = {:?}", fruits.get(&1));
    
    // removing value in a hashmap
    fruits.remove(&1);
    
    println!("First Fruits After Removing = {:?}", fruits.get(&1));
}
```

* Output : 

```plain
First Fruits Before Removing = Some("Apple")
First Fruits After Removing = None
```
* Here, we have use `&` ampersand to remove the values from a HashMap.

#### Changing Element of a HashMap in Rust 

* We can change/update elements of a hashmap by using the `insert()` method. For example,

```rust
use std::collections::HashMap;

fn main(){
    // create a new HashMap
    let mut fruits: HashMap<i32, String> = HashMap::new();

    // add key-value in a hashmap
    fruits.insert(1, String::from("Apple"));
    fruits.insert(2, String::from("orange"));

    println!("First Fruits Before Updating = {:?}", fruits.get(&1));
    
    // Update value in a hashmap
    fruits.insert(1, String::from("Strawberry"));
    
    println!("First Fruits After Updating = {:?}", fruits.get(&1));
}
```

* Output : 
```plain
First Fruits Before Updating = Some("Apple")
First Fruits After Updating = Some("Strawberry")
```

### Others Methods of Rust HashMap

|Method | Description |
|--------|------------|
| `len()` | Returns the length of the HashMap. |
| `contains_key()` | Checks if a value exists for the specified key. |
| `iter()` | Returns an iterator over the entries of a HashMap. |
| `values()` | Returns an iterator over the Keys of a HashMap. |
| `Keys()` | Returns an iterator over the keys of a HashMap. |
| `clone()` | Creates and return a copy of the HashMap. |

* Example : 

```rust
use std::collections::HashMap;

fn main(){
    // create a new HashMap
    let mut fruits: HashMap<i32, String> = HashMap::new();

    // add key-value in a hashmap
    fruits.insert(1, String::from("Apple"));
    fruits.insert(2, String::from("orange"));
    
    println!("Length of fruit hashmap {} ", fruits.len());
}
```
* Output : 
```plain
Length of fruit hashmap 2 
```







## Rust HashSet :

* HashSet implement the set data structure in Rust. Just like a set, it allows us to store values without duplicates.

### Creating a HashSet in Rust

* Before creating a `HashSet` we need to import it from rust standard collection library by :
```rust
use std::collections::HashSet;
```

* we can create a hashset by using `new()` method of the `HashSet` module.

```rust
use std::collections::HashSet;

fn main(){
    // create a new hashset
    let mut color: HashSet<String> = HashSet::new();

    println!("hashSet = {:?}", color);
}
```

* Here,
  * `let mut color` - declares a mutable  variable **color**.
  * `HashSet<String>` - type of the hashset where the value are of type **string**.
  * `HashSet::new()` - creates a new hashset.

### HashSet Operations in Rust 

* The `HashSet` module provide various methods to perform basic operations in a hashset. 
  * Add Values
  * Check Values
  * Remove Values
  * Iterate over Values

#### Add values to a HashSet in Rust : 
* We can use `insert()` method to add an element to the hashset. For example,
```rust
use std::collections::HashSet;

fn main(){
    let mut colors: HashSet<&str> = HashSet::new();

    // insert elements to hashset
    colors.insert("Red");
    colors.insert("Yellow");

    println!("Colors = {:?}", colors);
}
```
* Output :  `plain Colors = {"Yellow", "Red"}`

#### Check Value is Present in a HashSet in Rust : 

* We can use `contains()` method to check if a value is present in a hashset. For example,
```rust
use std::collections::HashSet;

fn main(){
    let mut colors: HashSet<&str> = HashSet::new();

    // insert elements to hashset
    colors.insert("Red");
    colors.insert("Yellow");

    // check for a value in HashSet
    if colors.contains("Red") {
        println!("Yes ");
    }
}
```
* Output : `Yes`

#### Remove Values from a HashSet in Rust

* We can use `remove()` method to remove the specific element from the hashset. For example,

```rust
use std::collections::HashSet;

fn main(){
    let mut colors: HashSet<&str> = HashSet::new();

    // insert elements to hashset
    colors.insert("Red");
    colors.insert("Yellow");

    println!("Colors before removing : {:?}",colors);
    
    // Remove value from a HashSet
    colors.remove("Yellow");
    
    println!("Colors after removing : {:?}",colors);
}
```
* Output : 
```plain
Colors before removing : {"Yellow", "Red"}
Colors after removing : {"Red"}
```

#### Iterate over Values of a HashSet in Rust :

* We can use **Rust For loop** to iterate over values of a hashset. For example,

```rust
use std::collections::HashSet;

fn main(){
    let mut colors: HashSet<&str> = HashSet::new();

    // insert elements to hashset
    colors.insert("Red");
    colors.insert("Yellow");
    colors.insert("Green");
    colors.insert("Black");
    colors.insert("Pink");

    // iterate over a hashset
    for i in colors {
        // print each value in the hashset
        println!("{}", i);
    }
}
```
* Output :

```plain
Yellow
Red
Green
Pink
Black
```

### HashSet with Default Values in Rust : 

* We can use `from()` method when creating it. For example,

```rust
use std::collections::HashSet;

fn main(){
    // creating hashset with default values : 
    let numbers = HashSet::from([2, 7, 9, 10]);

    println!("numbers = {:?}", numbers);
}
```
* Output : 
```plain
numbers = {2, 7, 10, 9}
```

### Other Methods of Rust HashSet 

| Method | Description |
|--------|-------------|
| `len()` | returns the length of a hashset |
| `is_empty()` | checks if the hashset is empty |
| `clear()` | removes all elements from the hashset |
| `drain()` | returns all the elements aas an iterator and clears the hashset |


### Set Operations 

#### Union of two Set :

* We can use `union()` method to find the union of two sets. For example,

```rust
use std::collections::HashSet;

fn main(){
   let hashset1 = HashSet::from([2, 7, 8]);
   let hashset2 = HashSet::from([1, 2, 7]);

   // Union of HashSets
   let result: HashSet<_> = hashset1.union(&hashset2).collect();

   println!("hashset1 = {:?}", hashset1);
   println!("hashset2 = {:?}", hashset2);
   println!("Union = {:?}", result);
}
```
* Output : 
```plain
hashset1 = {2, 7, 8}
hashset2 = {1, 2, 7}
Union = {2, 7, 1, 8}
```

* Here, `union()` method return an iterator, so we have used the `collect()` method to get the actual result.

* **NOTE :** We have passed `&hashset2` as an argument to the `union()` method because it takes a reference as an argument.

### Intersection of two sets : 

* We can use the `intersection()` method to find the intersection b/w two sets. For example : 

```rust
use std::collections::HashSet;

fn main(){
   let hashset1 = HashSet::from([2, 7, 8]);
   let hashset2 = HashSet::from([1, 2, 7]);

   // Intersection of HashSets
   let result: HashSet<_> = hashset1.intersection(&hashset2).collect();

   println!("hashset1 = {:?}", hashset1);
   println!("hashset2 = {:?}", hashset2);
   println!("Intersection = {:?}", result);
}
```

* Output :

```plain
hashset1 = {7, 2, 8}
hashset2 = {1, 7, 2}
Intersection = {2, 7}
```

### Difference between two Sets : 
* We can use `difference()` method to find the difference between two sets. For example,

```rust
use std::collections::HashSet;

fn main(){
   let hashset1 = HashSet::from([2, 7, 8]);
   let hashset2 = HashSet::from([1, 2, 7]);

   // Difference of HashSets
   let result: HashSet<_> = hashset1.difference(&hashset2).collect();

   println!("hashset1 = {:?}", hashset1);
   println!("hashset2 = {:?}", hashset2);
   println!("Difference = {:?}", result);
}
```
* Output : 
```plain
hashset1 = {7, 2, 8}
hashset2 = {2, 1, 7}
Difference = {8}
```

### Symmetric Difference between two Sets : 

* We can use the `symmetric_difference()` method to find the symmetric difference between two sets. For example

```rust
use std::collections::HashSet;

fn main(){
   let hashset1 = HashSet::from([2, 7, 8]);
   let hashset2 = HashSet::from([1, 2, 7, 9]);

   // Symmetric Difference of HashSets
   let result: HashSet<_> = hashset1.difference(&hashset2).collect();

   println!("hashset1 = {:?}", hashset1);
   println!("hashset2 = {:?}", hashset2);
   println!("Symmetric Difference = {:?}", result);
}
```

* Output : 

```plain
hashset1 = {2, 8, 7}
hashset2 = {1, 7, 2, 9}
Symmetric Difference = {8}
```







## Rust Iterator :

* An Iterator in rust is responsible for creating a sequence of value and allows us to iterate over each item of the sequences. It is primarily used for looping and we can only loop over iterator in Rust.
  
* We can use `iter()` method to create an iterator.
* Let's look at a simple example on how we can loop through an array.
 
```rust
fn main(){
    let numbers = [2, 1, 17, 99, 56];

   // iterator 
   let numbers_iterator = numbers.iter();

   for number in numbers_iterator {
       println!("{}", number);
   }
}
```

* Output : 

```plain
2
1
17
99
56
```

*  **NOTE :** Collection like array, vector, hashmap, and hashset are not iterable by default. We can use `iter()` method to tell Rust that it can be used to loop over the values.

### next() Method of an Iterator in Rust :

* Another important method of iterator is the `next()` method. The `next()` method of an iterator can be used to traverse through the value in the iterator.
* Every iterator in Rust by definition will have the `next()` method. The `next()` method is used to fetch individual value from the iterator. For example,

```rust
fn main(){
   let colors = vec!["Red", "Yellow", "Green" ];

   // iterator 
   let mut colors_iterator = colors.iter();
   println!("colors iterator = {:?}", colors_iterator);

   // fetch values from iterator one by one using next() method 
   println!("{:?}", colors_iterator.next());
   println!("{:?}", colors_iterator.next());
   println!("{:?}", colors_iterator.next());
   println!("{:?}", colors_iterator.next());
}
```

* Output : 
```plain
colors iterator = Iter(["Red", "Yellow", "Green"])
Some("Red")
Some("Yellow")
Some("Green")
None
```

* Here we fetch values from the iterator in `colors_iterator` using the `next()` method. The `next()` method either return `Some` value or `None`.
* Notice that we need to make the `colors_iterator` a mutable variable because calling `next()` will change the internal state of the iterator reaches the end of the sequence.

### Ways to Create iterator in Rust : 

* We can create iterator by converting a collection into an iterator. There are three ways to create an iterator.
  1. Using `iter()` method
  2. Using `into_iter()` method
  3. Using `iter_mut()` method.
  

#### 1. Using `iter()` method

* Using the `iter()` method on a collection will borrow(reference) each element of the collection in each iteration. Thus, the collection will be available for use after we have looped through it. For example,

```rust
fn main(){
   let colors = vec!["Red", "Green", "Blue"];

   // using the iter() to iterate through a collection
   for color in colors.iter() {
      // reference to the items in the iterator 
      println!("{}", color);
   }

   // the collection is untouched and still available here 
   println!("colors = {:?}", colors);
}
```
* Output : 
```plain
Red
Green
Blue
colors = ["Red", "Green", "Blue"]
```
* Notice here that the `colors` variable still available after `iter()` method is used on it.

#### 2. Using `into_iter()` method

* Using the `into_iter()` method on a collection will iterate on the same element of the collection in each iteration. Thus, the collection will no longer be available for reuse as the value moves within the loop.

```rust
fn main(){
   let colors = vec!["Red", "Green", "Blue"];

   // using the iter() to iterate through a collection
   for color in colors.into_iter() {
      // reference to the items in the iterator 
      println!("{}", color);
   }

   // error 
   // the collection is not available here as the loop scope ends above 
   println!("colors = {:?}", colors);
}
```
* Output : 

```plain
error[E0382]: borrow of moved value: `colors`
   --> src/main.rs:12:30
    |
2   |    let colors = vec!["Red", "Green", "Blue"];
    |        ------ move occurs because `colors` has type `Vec<&str>`, which does not implement the `Copy` trait
...
5   |    for color in colors.into_iter() {
    |                        ----------- `colors` moved due to this method call
...
12  |    println!("colors = {:?}", colors);
    |                              ^^^^^^ value borrowed here after move
    |
```

* Notice here that the `colors` variable is unavailable because the `into_iter()` method moves the actual data into the `for` loop and is not available outside of it's scope.


***
* **Note :** By default the for loop will apply the `into_iter()` function to the collection. We don't have to use the `into_iter()` function to convert the collection to an iterator when using the for loop.
* For example, these two ways to loop through an iterator are the same 
```rust
for color in colors.into_iter() {
    // code
}
for color in colors {
    // code 
}
```
* Example : 
```rust
fn main(){
   let colors = vec!["Red", "Green", "Blue"];

   for color in colors {
      println!("{}", color);
   }

   // error 
   // the collection is not available here as the loop scope ends above 
   println!("colors = {:?}", colors);
}
```
***

#### 3. Using `iter_mut()` method.

* Using the `iter_mut()` method on a collection will mutably borrow each element of the collection in each iteration. It means we can modify the collection in place. For example,

```rust
fn main(){
   let mut colors = vec!["Red", "Greed", "Yellow"];

   println!("Original value of colors = {:?}", colors);

   // using iter_mut() to iterate through a collection
   for color in colors.iter_mut(){
      // modify the item in the collection 
      *color = "Black";
      println!("{}", color);
   }

   // the modified collection is available here 
   println!("Modified value of colors = {:?}", colors);
}
```
* Output : 
```plain

Original value of colors = ["Red", "Greed", "Yellow"]
Black
Black
Black
Modified value of colors = ["Black", "Black", "Black"]
```

* Notice here that we use `iter_mut()` method to change the original item in the collection with `*color = "Black"`. Thus, every item in the collection after for loop is modified.
  
### Iterator Adapters in Rust 

* Iterator adapter are used to transform it into another kind of iterator by altering its behavior. let's take a look at the `map()` adapter.

```rust
let numbers = vec![1, 2, 3];

numbers.iter().map(|i| i + 1);
```

* Here, the `map()` method takes a closure to call on each item on the vector numbers.
* however, we will have to use the `collect()` method on the `map()` adapter to collect the result. This is because iterator adapter do not produce the result directly(lazy) without calling the `collect()` method.

```rust
numbers.iter().map(|i| i + 1).collect();
```
* This will return a vector containing each element from the original vector incremented by **1**.

* **Example : Iterator Adapter** 

```rust
fn main(){
   let numbers: Vec<i32> = vec![1, 2, 3];

   // using the map iterator adapter
   let even_number: Vec<i32> = numbers.iter().map(|i| i*2).collect();

   println!("Numbers = {:?}", numbers);
   println!("even_numbers = {:?}", even_number);
}
```

* Output : 

```plain
Numbers = [1, 2, 3]
even_numbers = [2, 4, 6]
```

  
### Range in Rust : 

* One of the other ways to create an iterator is to used the range notation. An example of a range is `1..6` which is an iterator. For example,
```rust
fn main(){
    // looping through a range
    for i in 1..6 {
        println!("{}", i);
    }
}
```
* Output : 
```plain
1
2
3
4
5
```











## Rust Error Handling : 

* An error is an unexpected behavior or event in a program that will produce an unwanted output.
* In Rust, errors are of two categories: 
  * Unrecoverable Errors
  * Recoverable Errors

### Unrecoverable Errors in Rust 

* Unrecoverable errors are errors from which a program stops it's execution. As the name suggest, we cannot recover from unrecoverable errors.
* These errors are known as **panic** and can be trigger explicitly by calling the `painc!` macro. Example,

```rust
fn main(){
   println!("Hello world!");

   // Explicitly exit the program with an unrecoverable error :
   panic!("Crash");
}
```
* Output : 

```plain
Hello world!
thread 'main' panicked at src/main.rs:5:4:
Crash
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```
* Here, the call to the `panic!` macro causes an unrecoverable error.
`thread 'main' panicked at src/main.rs:5:4:`
* Notice that the program will still runs the expression above `panic!` macro. We can still see `Hello World!` printed to the screen before the error message.
* The `painc!` macro takes in an error message as an argument.

### Example : Rust unrecoverable error : 

```rust
fn main(){
   let number = [1, 2, 3];

   println!("Unknown index value = {}", number[3]);
}
```
* Output : 
```plain
error: this operation will panic at runtime
 --> src/main.rs:4:41
  |
4 |    println!("Unknown index value = {}", number[3]);
  |                                         ^^^^^^^^^ index out of bounds: the length is 3 but the index is 3
  |
  = note: `#[deny(unconditional_panic)]` on by default
```
* Here, Rust stops us from compiling the program because it knowns the operations will panic at runtime.
* The array `number` does not have a value at index **3** i.e `number[3]`.

### Recoverable Errors : 

* Recoverable Errors are errors that won't stop a program from a execution. Most errors are recoverable, and we can easily take action based on the type of error.
* For example, if you try to open a file that doesn't exist, you can create the file instead of stopping the execution of the program or exiting the program with a panic. For example,
  
```rust
use std::fs::File;

fn main(){
   let data_result = File::open("data.txt");

   // using match for Result type
   let data_file = match data_result {
      Ok(file) => file,
      Err(error) => panic!("Problem opening the data file : {:?}", error),
   };

   println!("Data file : {:?}", data_file);
}
```

* If the `data.txt` file exist, the output is : 

```plain
Data file : File { fd: 3, path: "/Users/sahilwep/Developer/Rust/data.txt", read: true, write: false }
```
* If the  `data.txt` file doesn't exist, the output is : 

```plain
thread 'main' panicked at src/main.rs:9:21:
Problem opening the data file : Os { code: 2, kind: NotFound, message: "No such file or directory" }
```

### The Result Enum : 
* In the above example, the return type of the `File::open('data.txt)` is a `Result<T, E>`.
* The `Result<T, E>` type returns either a value or an error in Rust. It is an `enum` type with two possible variants.
  * `Ok(T)` -> Operation succeeded with value `T`
  * `Err(E)` -> Operation failed with an error `E`
* Here, `T` and `E` are **generic types**. We will discuss it later on.
* Th most basic way to see whether a `Result` enum has a value or error is use pattern matching with a `match` expression.

```rust
match data_result {
      Ok(file) => file,
      Err(error) => panic!("Problem opening the data file : {:?}", error),
   };
```

* When the result is `Ok`, this code will return the file, and when the result is `Err` it will return a `panic!`.
* We can study it on pattern matching later on.

### The Option Enum 

* The `Option` type or `Option<T>` type is an `enum` type just like `Result` with two possible variants.
  * `None` -> to indicate failure with no value
  * `Some(T)` -> a value with type `T`
* Let's look at an example :

```rust
fn main(){

   let text = "Hello world";

   let character_option = text.chars().nth(15);

   // using match for Options type
   let character = match character_option {
      None => "empty".to_string(),
      Some(c) => c.to_string()
   };

   println!("Character at index 15 is {}", character);
}
```
* Output :

```plain
Character at index 15 is empty
```
* Here, the method `text.chars().nth(15)` return an `Option<String>`. So, to get the value out of the `Option`, we use a `match` expression.

* In the example above, the 15th index of the string `text` doesn't exist. Thus, the `Option` type returns a `None` which matches the `"empty"` string.

```rust
None => "empty".to_string()
```

* If we have to get `11th` index of ths string `text`, the `Option` enum would return `Some(c)`, where `c` is the character in the `11th` index.

* Let's update the above example to find out the 11th index in the string.

```rust
fn main(){

   let text = "Hello, world!";

   let character_option = text.chars().nth(11);

   // using match for Options type
   let character = match character_option {
      None => "empty".to_string(),
      Some(c) => c.to_string()
   };

   println!("Character at index 11 is {}", character);
}
```
* Output : 
```plain
Character at index 11 is d
```

### Difference between Result and Option enum in Rust : 

* `Option` enum can return `None`, which can indicate failure.
* However, sometimes it is essential to express why an operation failed. Thus, we have the `Result` enum, which gives us the `Err` with the reason behind the failure of the operation.
* In short, 
  * `Option` is about `Some` or `None` (value or no value)
  * `Result` is about `Ok` or `Err` (result or error result)










## Rust unwrap() and expect()

* Unwrap in Rust returns the result the operations for `Option` and `Result` enums.
* If unwrap encounters an error `Err` or a `None`, it will panic and stop program execution.
* Unwrap method is defined on both `Option` and `Result` type.
* An `Option` enum type can be handled by using the `match` expression as well as `unwrap()`.

* **Example : Using the match expression**

```rust

// Function to find a user by their username which returns as Option type 
fn get_user(username: &str) -> Option<&str> {
   if username.is_empty() {
      return None;
   }
   return Some(username);
}

fn main(){
      // return an Option
      let user_option = get_user("Sahil");

      // user of match expression to get the result out of Option
      let result = match user_option {
         Some(user) => user,
         None => "not found!",
      };

      // print the result 
      println!("User = {:?}", result);
}
```
* If we pass some value inside the `user_option` it will return that value, else if there is no values, then it will return the `not found!`. 
* Here, we have a `get_user` function that returns an `Option` type. it can either return `Some(&str)` or `None`

* Now this program we can use the `unwrap()` method to get rid of the `match` expression which is a little verbose.
* **Let's use `unwrap()` in the above example.**

```rust

// Function to find a user by their username which returns as Option enum
fn get_user(username: &str) -> Option<&str> {
   if username.is_empty() {
      return None;
   }
   return Some(username);
}

fn main(){
      // use of unwrap method of get the result of Option enum from get_user function
      let result = get_user("Sahil").unwrap();

      // print the result 
      println!("User = {:?}", result);
}
```
* Output = "Sahil"
* Both the `match` expression and `unwrap()` gives us the same output. 
* The only difference being that `unwrap()` will panic if the return value is a None.
* If we update the above program to send an empty username argument to the `get_user()` method. it will panic.
```plain
thread 'main' panicked at src/main.rs:12:33:
called `Option::unwrap()` on a `None` value
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

### The `expect()` Method

* `expect()` is very similar to `unwrap()` with the addition a custom panic message as an argument.
* The `expect()` method is defined on both `Option` and `Result` type.
* Let's update the above example to use `expect()` instead of `unwrap()`.

```rust

// Function to find a user by their username which returns as Option enum
fn get_user(username: &str) -> Option<&str> {
   if username.is_empty() {
      return None;
   }
   return Some(username);
}

fn main(){
      // use of expect method of get the result of Option enum from get_user function
      let result = get_user("Sahil").expect("fetch user");

      // print the result 
      println!("User = {:?}", result);
}
```
* If we pass the argument to `get_user` method it will return this.

```plain
User = "Sahil"
```
* If we didn't pass any argument to the `get_user` method, it will return this.
```plain
thread 'main' panicked at src/main.rs:12:33:
fetch user
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

* Here, we use the `expect()` with a panic message as the argument.

* `expect()` and `unwrap()` will produce the same result if there's no possibility of `Options` returning `None` and `Result` returning `Err`.

* **Note :** `unwrap()` and `expect()` are utility method to work with `Option` and `Result` types. It makes our program concise and prevents the need to write verbose `match` expression to return a result.

### The Question Mark (?) Operator

* The question mark (`?`) operator is a shorthand for returning the `Result`. It can only be applied to `Result<T, E>` and `Option<T>` type.
* When we apply `?` to `Result<T, E>` type:
  * If the value is `Err(e)`, it returns an `Err()` immediately
  * If the value is `Ok(x)` it unwraps and returns `x`
* **Let's look at an example**
```rust
use std::num::ParseIntError;

// function to parse an integer
fn parse_int() -> Result<i32, ParseIntError> {
   // Example of ? where value is unwrapped
   let x: i32 = "12".parse()?; // x = 12

   // Example of ? where error is returned
   let y: i32 = "12a".parse()?;  // returns an Err() immediately

   Ok(x + y) // Doesn't reach this line
}

fn main() {
   let res = parse_int();

   println!("{:?}", res);
}
```

* Output : 

```plain
Err(ParseIntError { kind: InvalidDigit })
```

* This way, error handling in the function is reduced to a single line of code, making it cleaner and easier to read.
* Similarly, when we apply `?` to `Option<T>` type:
  * if the value is `None`, then it returns `None`
  * if the value is `Some(x)`, then it unwraps the value and return `x`

* **NOTE :** The question mark operator (`?`) can only be used in a function that return `Result` or `Option`.


## Rust Ownership : 

***
* **Variable Scope in Rust**

* Scope is a code block within the program for which a variable is valid. The scope of a variable define it's ownership.

```rust 
{// code block start here
    let name = String::from("Sahil");
    // we can do stuff with name
}// code block ends
// this scope ends, "name" is no longer valid and cannot be used.
```
***

### Ownership Rules in Rust : 

* Rust has some ownership rules. keep these rules in mind as we work through examples:
  1. Each value in Rust has an owner.
  2. There can only be one owner at a time.
  3. When the owner goes out of scope, the value will be dropped.

### Data Move in Rust : 

* Sometimes, we might not want a variable to be dropped at the end of the scope. Instead, we want to transfer ownership of an item from one binding(variable) to another.

* Here's an example to understand data movement and ownership rules in Rust.

```rust
fn main(){
   // owner of the string value
   // rule no. 1
   let fruit1 = String::from("Banana");

   // ownership moves to another variable
   // only one owner at a time
   // rule no. 2
   let fruit2 = fruit1;

   // cannot print variable fruit1 because ownership has moved
   // error, out of scope, value is dropped
   // rule no. 3
   // println!("Fruit 1 = {}", fruit1);

   // print value of fruit2 on the screen
   println!("Fruit 2 = {}", fruit2);
}
```
* Output : `Fruit 2 = Banana`

* Here, `Fruit1` is owner of the `String`.

* A String store data both on the stack and heap. This means that when we bind a `String` to a variable `fruit1`, the memory representation looks like this : 

![Memory Representation of a String holding the value "Banana" bound to fruit1](assets/img_1.png)

* A `String` holds a pointer to the memory that holds the content of the string, a length and a capacity in the stack. The heap on the right hand side of the diagram holds the contents of the `String`.

* Now, when we assign `fruit1` to `fruit2`, this is how the memory representation looks like:

![Memory Representation when String value "Banana" moves from fruit1 to fruit2](assets/img_2.png)

* Rust will invalidate (drop) the first variable `fruit1`, and move the value to another variable `fruit2` This way two variable cannot point to the same content. At any point, there is only one owner of the value.

* **NOTE :** The above concept is applicable for data types that don't have fixed size in memory and use the heap memory to store the contents.

### Data Copy in Rust : 

* Primitive type like `Integer`, `float` and `boolean` don't follow the `ownership rules`.
* These types have known `size` at compile time and are stored entirely on the `stack`, so copies of the actual values are quick to make. For example, 

```rust
fn main(){
   let x: u8  = 11;

   // copies data from x to y
   // ownership rules are not applied here.
   let y = x;

   println!("x = {}, y = {}", x, y);
}
```
* Output : `x = 11, y = 11`

* Here, `x` variable can be used afterward, unlike a move without worrying about ownership, even through `y` is assigned to `x`.

* This Copying is possible because of the `Copy` trait available in primitive types in Rust. When we assign `x` to `y`, a copy of the data is made.
* A `trait` is a way to define shared behavior in Rust. We will discuss afterwords.

### Ownership in Functions : 

* Passing a variable to a function will `move` or `copy`, just as an assignment. **Stack-only** type will copy the data when passed into a function. **Heap data** type will move the ownership of the variable to the function. Example : 

#### 1. Passing String to a function :

```rust
fn main(){
   let fruit = String::from("Mango");  // fruit comes into scope

   // ownership of fruit moves into the function
   print_fruit(fruit);

   // fruit is moved to the function so is no longer available here
   // error
   // println!("Fruit = {}", fruit);
}

fn print_fruit(frt: String) {
   println!("fruit is  = {}", frt);
} // frt goes out of scope and is dropped, plus memory is freed
```
* Output : `fruit is  = Mango`

#### 2. Passing Integer to a function :

```rust
fn main(){
   let num = 12;  // num comes into scope

   // ownership of num copied into the function
   print_num(num);

   // num variable can be used here
   println!("Fruit = {}", num);
}

fn print_num(num: i8) {
   println!("num is  = {}", num);
} // num goes out of scope
```
* Output : 

```plain
num is  = 12
Fruit = 12
```

* Here, the value of the number variable is copied into the function `print_number()` because the `i8`(integer) type used stack memory.











## Rust Referencing and Borrowing : 

* **References** in Rust allows us to point to a resource(value) without owning it. This means that the original owner of the resources remains the same.
* References are helpful when passing values to a function that we do not want to change the ownership of. **Creating a reference is known as `borrowing in Rust`.**

### Understanding References in Rust : 

* Let's look at an example to learn above references in Rust.

```rust
fn main(){
   let value = String::from("Sahilwep Github!");

   // call function with reference String value
   let len = calculate_length(&value);

   println!("Length of '{}' is {}.", value, len);
}

// function to calculate length of a string
// It takes a reference of a String as an argument
fn calculate_length(s: &String) ->  usize {
   return s.len();
}
```
* **Output :** `Length of 'Sahilwep Github!' is 16.`

* In the above example, we define a function called `calculate_length()` which takes a `&String` type as an argument.

* The important part here is that `s` is a reference to a `String` and it doesn't take ownership of the actual value of `String`.

```rust
fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len();
}
```
* When, `s` goes out of scope, at the end of the function, it is not dropped because it does not have ownership of what it refers to.

* The function call looks like:
```rust
let value  = String::from("Sahilwep Github!");

let len = calculate_length(&value);
```
* The `&value` syntax while calling the function lets us create a **reference** that refers to the value of `str` but does not own it.
* The action of creating a references is known as `borrowing`. Borrowing is when we borrow something, and we are done with it.
* **NOTE :** Ampersand (`&`) represent reference, and they allows us to refer to some value without taking ownership of it.
  

### Modifying a Reference in Rust : 

* By default a reference is always immutable. However, we can use the `&mut` keyword to make a reference mutable.
* For example,

```rust
fn main(){
   let mut str = String::from("sahil");
   
   // before modifying the string
   println!("Before: str = {}", str);

   // pass a mutable string when calling function.
   change(&mut str);

   // after modifying the string
   println!("After: str = {}", str);
}

fn change(s: &mut String){
   // push a string to the mutable reference variable
   s.push_str("wep");
}
```
* Output : 

```plain
Before: str = sahil
After: str = sahilwep
```

* Here, we set the variable `str` to be mutable. Then we create a mutable reference with `&mut str`, and call the `change()` function with a mutable reference `s: &mut String`.

* This allows the `change()` function to modify the value it borrows. inside the `change()` function, we push a string with `s.push_str("wep")` to the reference string.


*** 

* **NOTE :** If you have a mutable reference to a value, you can have no other reference to that value.

```rust
fn main(){
   let mut str = String::from("Sahil");

   // mutable reference 1
   let ref1 = &mut str;
   
   // mutable reference 2
   let ref2 = &mut str;

   println!("{}, {} ", ref1, ref2);
}
```
* Output : 

```plain
error[E0499]: cannot borrow `str` as mutable more than once at a time
  --> src/main.rs:8:15
   |
5  |    let ref1 = &mut str;
   |               -------- first mutable borrow occurs here
...
8  |    let ref2 = &mut str;
   |               ^^^^^^^^ second mutable borrow occurs here
9  |
10 |    println!("{}, {} ", ref1, ref2);
   |                        ---- first borrow later used here
```

***

### Rules of References : 

* Rust Primarily follows these rules of references at any given time:
  1. At any given time, you can have either one mutable reference or any number of immutable references.
  2. References must always be valid. 








## Rust Module : 


* Modules in Rust help in splitting a program into logical units for better readability and organization.

* Once a program gets larger, it is important to split into multiple files or namespaces. Modules help in structuring our program.

* A module is a collection of items: function, struct and even other modules.

### Defining a Module in Rust

* The `mod` keyword is used to define a module. The syntax of module is:

```rust
// syntax of a module
mod module_name{
    // code
}
```
* Here, `module_name` is the name of the module.

* Now, let's define a module.

```rust
// a module named config
mod config{
   // a function print inside of the module
   fn print() {
      println!("config!");
   }
}
```
* In the above example, we create a module named `config` using the `mod` keyword.
* Inside the module we can define multiple items. Here, we have defined the `print()` function.

### Visibility of items inside a Module in Rust

* Items inside a module can be private or public. By default, a module is private. It means items inside the module cannot be accessed outside of the module.
* The `pub` keyword can eb used to give an item public visibility. Example,

```rust
// a module named config
mod config{
   // items in module by default have private visibility   
   fn select() {
      println!("config!");
   }
   // use `pub` keyword to override private visibility
   pub fn print(){
      println!("config!");
   }
}
```
* If we compile our code, we don't get any output because we have not used the function yet.

* Now, let's call the functions inside the module.

```rust
// a module named config
mod config{
   // items in module by default have private visibility   
   fn select() {
      println!("config!");
   }
   // use `pub` keyword to override private visibility
   pub fn print(){
      println!("config!");
   }
}

fn main(){
    // public items inside module can be accessed outside the parent module
    // call public print function from config module
   config::print();
}
```
* Output : `config!`

* Here, we call the public function `print()` inside the `config` module using the syntax `config::print()`. The `::` operator  is used to separate the module name and the item to call inside the module.

* However, private items inside the module are not accessible outside the module. If we call the private function `select()` inside the `config` module, we get a complication error.

### Example: Using Module in Rust 

```rust
mod player {
   // private function
   fn focus(){
      println!("called player::focus");
   }
   // private function
   fn shift() {
      println!("called player::shift");
   }

   // public function
   pub fn jump() {
      // call private function focus and shift inside the module
      focus();
      shift();
      println!("called plyer::jump");
   }
}

fn main(){
   // call public function jump from player module
   player::jump();
}
```

* Output : 

```plain
called player::focus
called player::shift
called plyer::jump
```

### Nested Modules 

* A module can be defined inside another module. This is known as module nesting. example,

```rust
// nested module
pub mod player {
   pub mod sprite {
      pub fn create() {
         println!("Called player::sprite::create");
      }
   }
}

fn main(){
   // call public function create from sprite module which is inside player module
   player::sprite::create();
}
```
* Output : 

```plain
Called player::sprite::create
```

### The use keyword in Rust : 

* We can use the `use` keyword to bring item inside a module into the current scope. The `use` keyword helps us eliminate writing out the full module path to call functions.
* let's rewrite our nested module example with the help of the `use` keyword.

```rust
// nested module
pub mod player {
   pub mod sprite {
      pub fn create() {
         println!("Called player::sprite::create");
      }
   }
}

// bring the create function into scope
use player::sprite::create;

fn main(){
   // call public function directly
   create();
}
```

* Output : 

```plain
Called player::sprite::create
```

* Here, we use the `use` keyword to bring the `create()` function into the current scope from the `sprite` module which is inside the `player` module. This allows us to call the `create()` function directly, without having to fully qualify the name as `player::sprite::create()`








## Rust Crate and Package :

* A crate can contain one or more Rust modules, which is turn can contain code, such as functions, type and constants.
* A crate is of two types:
  * Binary create
  * Library crate

* A **binary create** is a Rust program that compiles to an executable or multiple executable and has a `main()` function for each executable.

* A **library crate** doesn't compile to an executable and doesn't have a `main()` function. A library crate generally defines a shared functionality that can be used in multiple projects.

* Crates can be bundled together into a package.

### Creating a Package in Rust 

* Packages can be created using the **Cargo package manager**, which is built into Rust. Cargo comes pre-install with Rust.

* We can use cargo to create a package. A **package** contains one or more crates that provide a set of functionality.

* **NOTE :** A package can contain many binary crates, but at most only one library crate.

#### Creating a Binary Package in Rust 

* To create a binary package, we can use the `cargo` command in the terminal.
```sh
$ cargo new hello_world --bin
```

* Output :

```plain
Created binary (application) `hello_world` package
```

* We create a binary package `hello_world` using `cargo` and the `--bin` option. it is the default cargo behavior.

* Let's look at the content of the `hello_world` package.
```plain
hello_world
├── Cargo.toml
└── src
    └── main.rs
```

* Here, `hello_world` is the package directory
* `Cargo.toml` is a file that contains metadata about the crate, such as it's name, version, and dependencies.
* `src/main.rs` is the crate root and contains the source code of the binary package.

#### Creating a Library Package in Rust

* Similarly, we can create a library package in Rust using cargo.
```sh
$ cargo new hello_world_lib --lib
```
* Output :
```plain
Created library `hello_world_lib` package
```

* We create a library package `hello_world_lib` using cargo and the `--lib` option. 
* Let's look at the contents of the `hello_world_lib` package.

```plain
hello_world_lib
├── Cargo.toml
└── src
    └── lib.rs
```

* Here, `hello_world_lib` is the package directory
* `Cargo.toml` is a file that contains metadata about the crate, such as its name, version and dependencies.
* `src/lib.rs` is the crate root and contains the source code of the library package.

* A package can contains `src/main.rs` and `src/lib.rs`. In this case, it has two crates: a binary and a library, both with the same name as the package. For example, ̰

```plain
hello_world
├── Cargo.toml
└── src
    └── lib.rs
    └── main.rs
```

* **Note :** Cargo by convention passes the crate root files to the Rust compiler to build the library or binary.





 


## Rust Cargo Package manager : 

* Cargo is the Rust package manager. it comes pre-installed with Rust and can be used ot package dependencies, manage them as well as build and distribute our won packages/libraries.

### Features of Cargo in Rust : 

* Cargo is a command line tool for rust which comes with these features: 
  * **Dependency management**
* Cargo makes it easy to manage the dependencies of our project, including adding, updating, and removing dependencies.
  * **Building and packaging**
* Cargo can automatically build and package our Rust projects, creating binary or library code that can be shared with others.
  * **Document generation**
* Cargo can automatically generate documentation for our code, making it easier for other developers to understand and use our library.
  * **Download crates**
    * Cargo can download and install libraries from crates.io, which is a central repository for Rust crates.
  * **Run a library or tests**
    * Cargo can build our source code, run the executable binary and also run our tests.
  
### Dependency Management with cargo in Rust :

* One of the primary feature of Cargo is that it can download, manage external libraries.
* We can visit **crates.io** to see how we can use use an external crates.
  * **crates.io** is a central repository where we can pull and publish shared libraries for Rust.

* When we create any project using cargo : 
```sh
$ cargo new hello_world
```
* We have `Cargo.toml` file in the root of our project directory `hello_world` is used to manage the dependencies.
* If we want to use the `rand` crate, we add the following line to the `[dependencies]` section of the `Cargo.toml`.
```toml
[package]
```
```toml
name = "hello_world"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
rand = "0.8.5"
```
* **Note :** We can also add dependency to our project using the command `cargo add rand`.
* Next, we'll need to import the crate to our `src/main.rs` Rust file. We can do this by using `extern crate <crate_name>` line at the top of the file.

```rust
extern crate rand;
```

* Now, we can use the "rand" crate to generate a random number between 1 and 6. The `use` keyword is used here to import items(such as functions, types, and constants) from the "rand" crate in the current scope.

```rust
extern crate rand;
```

```rust
use rand::Rng;

fn main(){
    let mut rng = rand::thread_rng();

    // simulate rolling a die
    println("roll = {}, rng.gen_range(1..=6));
}
```
* Output : `roll = 5`


### Building and Running Project with Cargo in Rust

#### Build the project

* We can use cargo to install, build and run our `hello_world` project with the "rand" crate. Here's how:

```sh
$ cargo build
```
* Output :
```sh
Compiling libc v0.2.139
Compiling cfg-if v1.0.0
Compiling ppv-lite86 v0.2.17
Compiling getrandom v0.2.8
Compiling rand_core v0.6.4
Compiling rand_chacha v0.3.1
Compiling rand v0.8.5
Compiling hello_world v0.1.0 (/path/rust/hello_world)
  Finished dev [unoptimized + debuginfo] target(s) in 2.57s
```
* The `cargo build` command first installs any crates that are in use inside the `src/` directory and then proceeds to compile the project.

#### Run the project

```sh
$ cargo run
```

* Output : 

```sh
Finished dev [unoptimized + debuginfo] target(s) in 0.05s
    Running `target/debug/hello_world`
roll = 5
```

### Useful cargo commands in Rust : 


| Command | Description |
|---------|-------------|
| `cargo new` | Create a new Rust project with basic directory structure |
| `cargo build` | Build(compile) the current project and generate a binary executable |
| `cargo run` | Build and run your current project( cargo build + run) |
| `cargo check` | Build the current project without generate a binary executable |
| `cargo add` | Add a new dependency and include it in `Cargo.toml` file |
| `cargo update` | Update all dependencies of current project to latest version |











## Rust Pattern Matching : 

* Pattern matching is a way to match the structure of a value and blind variable to its parts. it is a powerful way to handel data and control flow of a Rust program.
* We generally use the `match` expression when it comes to pattern matching.
* The syntax of the `match expression is : 
```rust
match VALUE {
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
}
```
* Here, `PATTERN => EXPRESSION` are called pattern, a special syntax in Rust which usually works together with the `match` keyword.

* **NOTE :** It is similar to **switch-case** but way more powerful.


### Matching a variable in Rust

* We can pattern match against the value of a variable. This is useful if our code wants to take some action based on a particular value. For example,

```rust
fn main(){
   let x = 2;

   // use match expression to pattern match against variable x
   match x {
      1 => println!("x is 1"),
      2 => println!("x is 2"),
      _ => println!("x is something else"),
   }
}
```
* Output : `x is 2`


### Matching an Enum in Rust : 

```rust
fn main(){
   enum Color {
      Red,
      Green,
      Blue,
   }

   let my_color = Color::Red;

   // use match expression to pattern match against variable x
   match my_color {
      Color::Red => println!("The color is red"),
      Color::Green => println!("The color is green"),
      Color::Blue => println!("The color is Blue"),
   }
}
```

* Output : `The color is red`

### Matching Option and Result Type in Rust :

* The most common case for pattern matching is with `Option` and `Result` enum types. Both the `Option` and `Result` type have two variants.

* `Option` type has:
  * `None` -> to indicate failure with no value
  * `Some(T)` -> a value with type `T`
* `Result` type has:
  * `Ok(T)` -> operation succeeded with value T
  * `Err(E)` -> operation failed with an error E
* Let's look at example of how we can sue pattern matching on these types.



*** 

#### Difference b/w Option and Result enum in Rust : 

##### Option : 
* The Option enum is used to express the concept of an optional value. It has two variants:
  1. `Some(T)`: Represents a value of type T.
  2. `None`: Represents the absence of a value.

* Here's an example of using `Option`:
```rust
// Option enum definition
enum Option<T> {
    Some(T),
    None,
}

// Example usage
fn divide(x: f64, y: f64) -> Option<f64> {
    if y != 0.0 {
        Some(x / y)
    } else {
        None
    }
}

fn main() {
    let result = divide(10.0, 2.0);

    match result {
        Some(quotient) => println!("Result: {}", quotient),
        None => println!("Cannot divide by zero"),
    }
}
```

* In this example, the divide function returns an `Option<f64>`, and the `match` statement is used to handle both cases (`Some` and `None`).


##### Result

* The `Result` enum is used to represent the result of an operation that can either succeed or fail. It has two variants:
  1. `Ok(T)`: Represents a successful result with a value of type `T`.
  2. `Err(E)`: Represents an error with a value of type `E`.

* Here's an example of using `Result`:

```rust
// Result enum definition
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// Example usage
fn divide(x: f64, y: f64) -> Result<f64, &'static str> {
    if y != 0.0 {
        Ok(x / y)
    } else {
        Err("Cannot divide by zero")
    }
}

fn main() {
    let result = divide(10.0, 0.0);

    match result {
        Ok(quotient) => println!("Result: {}", quotient),
        Err(err) => println!("Error: {}", err),
    }
}
```
* In this example, the `divide` function returns a` Result<f64, &'static str>`, where the error type is a string slice. The `match` statement is used to handle both successful and error cases.

* Using `Option` and `Result` helps in writing more expressive and safe code, as it forces the programmer to explicitly handle cases where a value may be absent (`Option`) or an operation may fail (`Result`). This approach can reduce the likelihood of runtime errors and improve code reliability.

*** 


#### Example: Matching Option Type in Rust 

* For Handling `Some(value)`
```rust
fn main(){
   let my_option: Option<i32> = Some(222);

   // use of match expression to match Option type 
   match my_option {
      Some(value) => println!("The option has a value of {}", value),
      None => println!("The Option has no value "),
   }
}
```
* Output : `The option has a value of 222`

* For Handling `None`
```rust
fn main(){
   let my_option: Option<i32> = None;

   // use of match expression to match Option type 
   match my_option {
      Some(value) => println!("The option has a value of {}", value),
      None => println!("The Option has no value "),
   }
}
```
* Output : `The Option has no value`


#### Example: Matching Result Type in Rust

* For Handling `Ok(value)`
```rust
fn main(){
   let my_option: Result<i32, &str> = Ok(100);

   // use of match expression to match Result type 
   match my_option {
      Ok(value) => println!("The Result is {}", value),
      Err(error) => println!("The error message is {} ", error),
   }
}
```
* Output : `The Result is 100`

* For Handling `Err(error_value)`: 
```rust
fn main(){
   let my_option: Result<i32, &str> = Err("e11h");

   // use of match expression to match Result type 
   match my_option {
      Ok(value) => println!("The Result is {}", value),
      Err(error) => println!("The error message is {} ", error),
   }
}
```
* Output : `The error message is e11h`


### `if let` Expression in Rust :

* The `if let` expression in Rust is a shorthand for a `match` expression with only one pattern/arm to match.

* It allows us to match on a value and then execute code only if the match is successful.

```rust
fn main(){
   let my_option: Option<i32> = Some(222);

   // use of if let expression on the Option type
   if let Some(value) = my_option {
      println!("The option has a value of {}", value);
   } else {
      println!("The option has no value ");
   }
}
```
* Output : `The option has a value of 222`

* Here, the `if let` expression is matching on the `my_option` variable and binding the value of `Some` variant to the `value` variable.
  
* If the match is successful, the code inside the `if` block is executed. if the match is not successful, the code inside the `else` block is executed.

### Common Use of Pattern Matching in Rust
* As you have seen, pattern matching is useful in numerous situation. Some common use cases for pattern matching include:
  * Matching against any value like integer.
  * Matching against enum, struct or tuple.
  * Expressing conditional logic
  * Destructing data structure or destructing element of an iterator in a for loop
  * Error handling with Result and Option types.








## Rust Generics :

* Generics allows us to write code that is flexible and be reuse with different types of data, without having to write separate implementations for each type. It helps us write code that can handle values of any type in a type-safe and efficient way.

* With the help of generics, we can define placeholder type for our methods, function, structs, enums and traits.

### Using Generics in Rust 

* We can understand generics by taking a look at [Rust Hashmap](#rust-hashmap).

* `HashMap` uses generics which allows creation of reusable and efficient code, as a single implementation that work with different types.
* A Rust `HashMap` has two `generic types`, one for the `key` and the second for the `value`.
* A HashMap type looks like this:

```rust
HashMap<K, V>
```
* Where `<K, V>`: `K` is the type of key and `V` is the type of the value.
* Now, when we create a HashMap we can set any type to `K` and `K`.

```rust
let mut numbers: HashMap<i32, &str> = HashMap::new();
```
* Here, the angle bracket `<i32, &str>` notation denotes the type of key and type of value of the HashMap. The type of the Key `K` is `i32` and the type of the value `V` is `&str`.

* Similarly, we create a HashMap and set the type of both key and value to `&str`.

```rust
let mut language_codes: HashMap<&str, &str> = HashMap::new();
```

* Using generics to define the type of HashMap helps us work with numerous arbitrary type available in Rust.
  

* **NOTE :**
  * Generics or generic type use a single character like `K`,`V`,`T`,`U` to distinguish from actual concrete type like `String`, `&str`, `i32`.
* As a convention,
  * `T`, `U` are used for arbitrary types
  * `K`, `V` are used for key-value types
  * `E` is used for error type

### Example: Using Generics in Rust

```rust
use std::collections::HashMap;

fn main(){
   // create a HashMap with type i32 and &str
   let mut numbers: HashMap<i32, &str> = HashMap::new();

   // insert values to numbers HashMap
   numbers.insert(1, "One");
   numbers.insert(2, "Two");

   println!("Numbers: {:?}", numbers);

   // create a HashMap with types &str and &str
   let mut language_code: HashMap<&str, &str> = HashMap::new();

   // Insert values to language_codes HashMap
   language_code.insert("EN", "English");
   language_code.insert("RU", "Russian");
   language_code.insert("HI", "Hindi");

   println!("Language Codes: {:?}", language_code);
}
```

* Output : 

```plain
Numbers: {1: "One", 2: "Two"}
Language Codes: {"EN": "English", "RU": "Russian", "HI": "Hindi"}
```

* Here, we create two HashMap data structure: `HashMap<i32, &str>` and `HashMap<&str, &str>`.

* This is possible because HashMap implementation uses generics and work with different types.

### Generic Struct in Rust

* We can create a generic struct data structure in Rust with the help of generics. For example, we can declare a struct with **generic parameter(s)**.

```rust
struct Point<T>{
    x: T,
    y: T,
}
```
* Here, we create a struct `Point` with generic type parameter `T` in angle brackets. Inside the body of the struct, we use the `T` data type for `x` and `y`.

* Now, to use the generic struct `Point` we can initialize it and bind it to a variable.

```rust
let int_point = Point {x: 1, y: 2};
let float_point = Point(x: 1.3, y: 42.2);
```
* Here, we initialize the `Point` struct twice, first with integer values and second with float values.

#### Example: Generic Struct in Rust


```rust
fn main(){
   // define a struct with generic data type
   #[derive(Debug)]
   struct Point<T> {
      x: T,
      y: T,
   }

   // initializing a generic struct with i32 data type
   let int_point = Point {x: 1, y: 2};
   
   // initializing a generic struct with f32 data type
   let float_point = Point {x: 1.2, y: 2.3};

   println!("int_point = {:?}", int_point);
   println!("float_point = {:?}", float_point);
}
```

* Output:

```plain
int_point = Point { x: 1, y: 2 }
float_point = Point { x: 1.2, y: 2.3 }
```



### Generic Function in Rust

* We can also create function with generic types as parameter(s).
* Here is the syntax of a generic function.

```rust
// generic function with  single generic type
fn my_function<T>(x: T, y: T) -> T {
   // function body
   // do something with `x` and `y`
}

// generic function with multiple generic types
fn my_function<T, U> (x: T, y: U) {
   // function body
   // do something with `x` and `y`
}
```

* Here, `<T>` in the function definition signifies a generic function over type `T`. Similarly, `<T, U>` signifies a generic function over type `T` and `U`.

#### Example: Generic Function in Rust

```rust

fn main(){
   // generic function to find minimum b/w two inputs
   fn min<T: PartialOrd> (a: T, b: T ) -> T {
      if a < b {
         return a;
      } else {
         return b;
      }
   }
   // call generic function with integer type as parameters
   let result1 = min(2, 7);

   // call generic function with float type as parameters
   let result2 = min(2.1, 3.1);

   println!("Result1 = {}", result1);
   println!("Result2 = {}", result2);
}
```

* Output : 
```plain
Result1 = 2
Result2 = 2.1
```









## Rust Trait :

* A Rust trait defines shared functionality for multiple types.

* Rust traits promote type-safety, prevent errors at compile time, and act like interfaces in other languages with some distinctions.

* One of the most common uses of traits is in function and method parameters. They allow functions and methods to accept parameters of different types.

* In Rust, traits are a way to define shared behavior across types. Traits provide a mechanism for declaring and grouping methods that can be implemented by different types. They are similar to interfaces or abstract classes in other programming languages.

### Defining a Trait in Rust 

* We can define a Rust trait using the `trait` keyword followed by the trait name and the method that are part of the trait.

* Let's look at the syntax of a trait.

```rust
trait TraitName {
   fn method_one(&self, [arguments: argument_type]) -> return_type;
   fn method_two(&mut self, [arguments: argument_type]) -> return_type
}
```
* Here, 
  * `TraitName` - name of the trait.
  * `method_one()` and `method_two()` - names of the methods in the trait.
  * `&self` and `&mut self` - references to the self value. A method can take either a mutable or immutable reference to the current object, depending on wether it needs to modify its value.
  * `[arguments: arguments_type]` (optional) - list of arguments, where each arguments has a name and a type.
  * `return_type` - type that method return.

* Now, let's define a trait.

```rust
trait MyTrait {
    fn method_one(&self);
    fn method_two(&mut self, arg: i32) -> bool;
}
```
* Here, we declare a trait called `MyTrait` with method signatures for `method_one(&self)` and `method_two(&mut self, arg: i32) -> bool`. The method signature describe the behaviors of hte types that implements his trait. 
* A trait can have multiple method signatures in its body, one per line. Traits by default do nothing and only are definitions. In order to use a trait, a type needs to implement it.


### Implementing a Trait in Rust : 

* To implement a trait, we use the `impl` keyword. The syntax for the implementation (impl) block is:


```rust
impl TraitName for TypeName {
    fn method_one(&self, [arguments: arguments_type]) -> return_type {
        // implementation for method_one
    }
    fn method_two(&mut self, [arguments: arguments_type]) -> return_type {
        // implementation for method_two
    }
    ...
}
```
* Here, `TraitName` is the name of the trait being implemented and `TypeName` is the name of the type that is implementing the trait.

* **NOTE :** The implementation of a trait must have the same signature as the method in the trait, including the name, the argument types, and the return type.

* Now, let's implemented the trait. We will use the `MyTrait` as the trait and `MyStruct` as the type for we implemented the trait.

```rust
// trait definition : 
trait MyTrait {
   // method signatures
   fn method_one(&self);
   fn method_two(&mut self, arg: i32) -> bool;
}

struct MyStruct {
   value: i32;
}

impl MyTrait for MyStruct {
   // implementation of method_one 
   fn method_one(&self) {
      println!("The value is: {}", self.value);
   }

   // implementation of method_two
   fn method_two(&mut self, arg: i32) -> bool {
      if arg > 0 {
         self.value += arg;
         return true;
      } else {
         return false;
      }
   }
}
```

* In this example, 
  * `method_one()` - takes reference to self and print its value `self.value` field.
  * `method_two()` - takes a mutable reference to self and an argument `arg` of type `i32`. if `arg` is grater than zero, and add `arg` to the value filed and return `true`, otherwise we return `false`.

### Example: Defining, implementing and Using a Trait in Rust 

```rust
// define a trait Printable
trait Printable {
   fn print(&self);
}

// define a struct to implement a trait
struct Person {
   name: String,
   age: u32,
}

// implement trait Printable on struct Person
impl Printable for Person {
   fn print(&self) {
      println!("Person {{ name: {}, age: {} }}", self.name, self.age);
   }
}

// define another struct to implement a trait
struct Car {
   make: String,
   model: String,
}

// define trait Printable on struct Car

impl Printable for Car {
   fn print(&self) {
      println!("Car {{ make: {}, model: {} }}", self.make, self.model);
   }
}

// Utility function to print any object that implements the Printable trait
fn print_things<T: Printable> (thing: &T) {
   thing.print();
}

fn main(){
   // instantiate Person and Car
   let person = Person {
      name: "sahilwep".to_string(), 
      age: 22
   };
   let car = Car {make: "Tesla".to_string(), model: "Model X".to_string()};      // .to_string() -> convert value to string.

   // Call print_thing with reference of Person and Car
   print_things(&person);
   print_things(&car);

}
```

* Output : 

```plain
Person { name: sahilwep, age: 22 }
Car { make: Tesla, model: Model X }
```

* In This example, we define a `Printable` trait and implement it two structs: `Person` and `Car`. The `Printable` trait requires the method name `print` for implementers.

* We have created a function `print_things` that is of generic type.

* In the `main()` function, we instantiate `Person` and `Car`, and pass it to the `print_thing()` function. The `print_thing` is a generic function that can accept reference to any object that implements the `Printable` trait.


### Default implementation of a Trait in Rust : 

* Sometimes it's useful to have default behavior for some or all of the method in a trait. When we defining a Rust trait, we can also define a default implementation of the methods.

* For example,

```rust
trait MyTrait {
    // method with default default implementation
    fn method_one(&self) {
        println!("Inside method_one");
    }

    // method without a default implementation
    fn method_two(&self, arg: i32) -> bool;
}
```

* Here, `method_one()` has a `println!()` function call inside of the `method_one()` body which acts as a default behavior for all types that implement the trait `MyTrait`.

* However, `method_two()` just defined the method signature.


### The derive keyword in Rust : 

* The `derive` keyword in Rust is used to generate implementation for certain trait for a type. it can be used in a `struct` or `enum` definition.


* Let's look at an example,


```rust
// use derive keyword to generate implementation of Copy and Clone
#[derive(Copy, Clone)]
struct MyStruct {
    value: i32,
}

fn main(){
    let x = MyStruct{value: 20};
    let y = x;
    
    println!("x: {:?}", x.value);
    println!("x: {:?}", y.value);
}
```
* Output : 
```plain
x: 20
x: 20
```

* Here, we use the `derive` keyword to implement trait `Copy` and `Clone` from the Rust standard library.

* The `Copy` trait allows us to assign `x` to `y` by simply copying. The `Clone` trait allows us to create a new instances that is an exact copy of an existing instance. 

* By using the `derive` keyword, we can avoid writing the code required to implement these traits.










## Rust File Handling : 

* File handling is a way to deal with data in file. file handling enables us to open, read, write, and update file on the local system.

* File handling is generally known as File I/O.


### File Struct in Rust : 

* In Rust, the `std::fs::File` struct represent a file. It allows us to perform read/write operations on a file.
* The file I/O is performed through the `std::fs` module which provides functions for working with the file system.
* All methods in the `File` struct return a variant of the `std::io::Result` or simply the `Result` enum.
* Let's look at the basics of file I/O in Rust with these operations:
  * **Opening a file**
  * **Reading from a file**
  * **Writing to a file**
  * **Removing a file**
  * **Appending to a file**


### Opening a File in Rust :

* To open a file in Rust, we use the `File::open()` method. This method takes a file path as an argument and return a `File` object. if the file does not exist, it returns an error(`Err`).
* Let's look at an example.

```rust
use std::fs::File;

fn main(){
   // Open a file in read only mode in the local file system
   let data_result = File::open("file.txt");

   // Reading a file return a Result enum
   // Result can be a file or an error 
   let data_file = match data_result {
      Ok(file) => file,
      Err(error) => panic!("Problem opening the data file: {:?} ", error),
   };

   println!("Data File: {:?} ", data_file);
}
```

* Output : 

```plain
Data File: File { fd: 3, path: "/path/rust/file.txt", read: true, write: false }
```
* Here, we import the module `std::fs::File` on the top of the program to use the file I/O functions.
* To open a file, we call `File::open("file.txt")` which reads the `file.txt` file in the local file system.
* The `open()` function returns a `Result` enum which will return the `File` object or an `Err`.
* Then, we pattern match the `data_result` variable and `panic!` if there is an error with opening the file. if opening the file doesn't error, we output the `File` object.

### Reading a File in Rust :

* To read a file in Rust, we use the `read_to_string()` method of the `std::io::Read` trait. This method reads all bytes until end of the file (EOF) and copies it to a mutable string.
* Here's an example.

```rust
use std::fs::File;
use std::io::Read;

fn main(){
   // read a file in the local file system
   let mut data_file = File::open("file.txt").unwrap();

   // Create an empty mutable string
   let mut file_content = String::new();

   // Copy contents of file to a mutable string
   data_file.read_to_string(&mut file_content).unwrap();

   println!("File content: {:?}", file_content);
}
```
* Output : 

```plain
File content: "hello\n"
```

* Here, we import two modules: `std::fs::File` and `std::io::Read` for reading a file. We first open the file `data.txt` with `File::open("file.txt")` method call and bind its result to a variable `data_file`.

* Once we open the file, we use the `read_to_string()` method which takes an empty mutable string `file_content` as an argument and copies the content of the file `file.txt` to `file_content`.

* Note: 
  * We use `unwrap()` twice to get the result from the method calls. `unwrap()` is a utility method to work with `Option` and `Result` type.
  * `read_to_string()` comes from the `std::io::Read` trait.

### Writing to a File in Rust :

* To write to a file in Rust, we can use the `write()` method from the `std::io::Write` trait. This method writes content to a file.
* Example,

```rust
use std::fs::File;
use std::io::Write;

fn main(){
   // create a file 
   let mut data_file = File::create("data.txt").expect("Creation failed");

   // write content to the file
   data_file.write("Hello world".as_bytes()).expect("Write Failed");

   println!("Created a file data.txt");
}
```
* Output : 

```plain
Created a file data.txt
```

* Here, we import `std::fs::File` and `std::io::Write` module for writing to a file. We first create a file `data.txt` with the `File::create("data.txt")` method and bind it to a mutable variable `data_file`.

* After we create a file, we write to the file using the `write()` method with the content `"Hello World"`.

### Removing a File in Rust :

* To remove or delete a file in Rust, we can use the `remove_file()` method from the `std::fs` module.
* For example,

```rust
use std::fs;

fn main(){
   // Remove a file 
   fs::remove_file("data.txt").expect("could not remove file");

   println!("Removed file data.txt");
}
```
* Output : 

```plain
Removed file data.txt
```

* Here, we import the `std::fs` module for deleting a file. We use the `remove_file()` method to delete the file `data.txt`.

* If the operations does not proceed, we return a custom message.
* If the file `data.txt` not found or cannot be removed, we encounter an error.

```plain
thread 'main' panicked at src/main.rs:5:32:
could not remove file: Os { code: 2, kind: NotFound, message: "No such file or directory" }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

### Appending to a File in Rust : 

* To append to a file in Rust, we should open the file in append mode. We can use the `append()` method in `std::fs::OpenOptions` which opens a file for appending. 
* Then, we can use the `write()` method in `std::io::Write` trait to write data to the file. example,

```rust
use std::fs::OpenOptions;
use std::io::Write;

fn main(){
   // open a file with append options
   let mut data_file = OpenOptions::new()
      .append(true)
      .open("file.txt")
      .expect("cannot open file");

   // Write to a file
   data_file
      .write("I am learning Rust!".as_bytes())
      .expect("Write failed");

   println!("Append content to a file");
}
```

* Output : 

```plain
Append content to a file
```

* Here, we import the `std::fs::OpenOptions` and `std::io::Write` module for appending to a file.
* The `OpenOptions::new()` and `append(true)` method open the file `file.txt` for appending.
* Next, we use the `write()` method from the `File` object to write additional content `I am learning Rust!` to the file.
* To deal with the errors we chain the `expect()` method with a custom error message.



### Example: All files operations : 


```rust
use std::fs::File;
use std::io::Write;
use std::io::Read;
use std::fs::OpenOptions;
use std::fs;

fn main(){

   // open a file
   let mut data_file = File::open("file.txt").unwrap(); // open the file and return file object, open() return a "Result" enum.
   println!("Data File: {:?}", data_file);  

   // reading a file
   let mut file_content = String::new();  // create an empty mutable string
   data_file.read_to_string(&mut file_content).unwrap(); // copy the content of file to a mutable string

   println!("File Content: {:?} ", file_content);

   // appending a file : means, writing into an existing file
   let mut data_file_temp = OpenOptions::new()
      .append(true)
      .open("file.txt")
      .expect("Not opened");

   data_file_temp
      .write("\nmy name is sahil".as_bytes())
      .expect("Write failed");

   println!("Appended content to a file");


   // writing a file : means creating a file 
   let mut data_file_new = File::create("data.txt").expect("Creation failed");      // creation of new file

   data_file_new.write("hello world!".as_bytes()).expect("Write failed");  // write content to a file
   println!("Created a file data.txt");
   // removing a file 

   fs::remove_file("data.txt").expect("Could not remove file");

   println!("Removed file data.txt");
}
```

* Here, we have used the several imports for performing all the file operations. First we open the file, then we write read the content of the file, then we append some content into the existing file, then we created a new file by writing some content into the file, then we delete the file.










## Rust Macro : 

* A macro in Rust is a piece of code that generates another piece of code.

* Macro generate code based on input, simplify repetitive pattern and makes code more concise.

* Rust macro simply allows us to write code that writes more code which is also known as meta programming. Macros are used extensively in Rust.

* Some of the popular Rust macros are `println!`, `vec!` and `panic!`.


### Creating a Macro in Rust :

* We can create a macro using the `macro_rules!` macro. it might be surprising but yes we use macro to create a macro.

* The `macro_rules!` macro has a special syntax.

* Syntax : 

```rust
macro_rule! macro_name {
    (...) => {...}
    // more match rule
}
```

* Here, `() => {}` is the entity for a macro rule. We can have many rules to match for a single macro.

* Example : 

```rust
// A simple macro named `hi_rust`
macro_rules! hi_rust {
   // `()` indicate that macro takes no argument
   () => {
      // the macro will expand into the contents of this block
      println!("Welcome to rust world!");
   };
}

fn main(){
   // call the hi_rust macro
   // this call will expand into `println!(...)`
   hi_rust!();
}
```

* Output : 

```plain
Welcome to rust world!
```
* In this example, we create a macro named `hi_rust`. The macro definition has one rule to match which is:

```rust
() => {
    println!(...);
}
```
* To call the macro we use the `hi_rust!()` call in the `main()` function.
* The macro will replace the `hi_rust()` call with the code defined in the macro definition i.e `println!("...")`.

### Creating a Macro with Arguments in Rust : 

* Macros can take arguments, which allows us to customize the code that it generates based on different inputs. Example,

```rust
// A macro named 'print_message'
macro_rules! print_message {
   // match rule that takes an argument expression
   ($message: expr) => {
      println!("message is : {}", $message)
   };
}

fn main(){
   // call the macro with an argument
   print_message!("I am learning rust!");
}
```

* Output :
```plain
message is : I am learning rust!
```
* Here, we create a macro called `print_message` which takes an argument `$message`. The argument(s) of a macro are prefixed by a dollar sign `$` and type annotated with a **designator**.

* Rust will try to match the pattern defined within the match rules. In the above example our rule is:

```rust
($message: expr) => {
    println("{}", $message)
}
```

* **NOTE :** The part after the semicolon `:` is called designator, which are types that we can choose to match for. We are using the expression designator(`expr`) in the example.

* Now, when we call the macro `print_message!("...")` it matches our input expression and captures the `$message` variable.

* Here, `$message` is assigned to `I am learning rust!` which is then passed into `println!("{}", $message)`.

* It will generate code that is equivalent to writing `println!("{}", "I am learning rust!")`. The `$message` argument allows us to specify the message to print.
* **NOTE :** There are many designator that we can use inside a macro rule body:
  * `stmt`: a statement
  * `pat`: a pattern
  * `expr`: an expression
  * `ty`: a type
  * `ident`: an identifier
  *  ...
  

### Macro Repetition in Rust 

* Rust macro is also useful when we need to generate repetitive code. We can define a macro to accept arguments and repeat the generated code based on those arguments.

* The `macro_rules!` macro supports repetition using the `$(...)*` syntax. The `...` inside the parenthesis can be valid Rust expression or a pattern.

```rust
// A macro which uses repetitions
macro_rules! repeat_print {
   // match rule which matches multiple expression in an argument
   ($($x: expr), *) => {
      $(
         println!("{}", $x);
      )*
   };
}


fn main(){
   // call the macro with multiple arguments
   repeat_print!(1, 2, 3, 4);
}
```
* Output : 
```plain
1
2
3
```

* Here, the macro `repeat_print` takes a single argument, `$(($x: expr), *)`, which is a repeating pattern.
* The pattern consists of zero or more expressions, separated by commas, that are matched by the macro. The star (`*`) symbol at the end will repeatedly match against the pattern inside `$()`.

* ![Breakdown of macro match rule in Rust](assets/img_3.png)

* The code inside the curly braces `println!("{}", $x)` is repeated zero or more times, once for each expression in the list of arguments as it is wrapped around `$(...)*` in the macro definition. The `$x` is the code refers to the matched expressions.

* Each iteration of the generated code will print a different expression. Now, when we call `repeat_print!(1, 2, 3);` the macro will generate this code:

```plain
println!("{}", 1); // matches argument 1,
println!("{}", 2); // matches argument 2,
println!("{}", 3); // matches argument 3
```

* Thus, this macro `repeat_print()!` can print multiple expression in a concise and convenient manner, without having to write out the `println!` macro every time.

### Rust Macro with return value : 

* We can use return value in macro definition : 
```rust

macro_rules! match_value {
    ($a: expr, $b: expr) => {
        {
            let mut res;
            if $a == $b {
                res = 1;
            } else {
                res = 0;
            }
            res
        };
    }
}

fn main(){
    let a: i32 = 3;
    let b: i32 = 4;

    let v = match_value!(a, b);

    println!("res = {v} ");

}
```

* Here, in this code we used `res` variable to return from the `match_value!` macro.



## Rust Threads :


* A thread is the smaller executable unit of a process.

* Threads allows us to split the computation in our program into multiple threads. Running multiple tasks at the same times can improve performance of the code.
* However, it can add complexity.

### Creating a New Thread in Rust :

* In Rust, we can create a native operating system thread using the `thread::spawn()` function from the `std` module. The spawn method takes a closure as an argument.

* Here is the syntax of `thread::spawn()`

```rust
thread::spawn( || {
    // code to execute in the thread
})
```

* Now, let's see an example.

```rust
use std::thread;
use std::time::Duration;

fn main(){
    // create a thread
    thread::spawn( || {
        // everything in here runs in a separate thread
        for i in 0..10 {
            println!("{} from the spawned thread! ", i);
            thread::sleep(Duration::from_millis(2));
        }
    });

    // main thread
    for i in 0..5 {
        println!("{} from the main thread! ", i);
        thread::sleep(Duration::from_millis(1));
    }
}
```

* Output : 

```plain
0 from the main thread! 
0 from the spawned thread! 
1 from the main thread! 
1 from the spawned thread! 
2 from the main thread! 
3 from the main thread! 
2 from the spawned thread! 
4 from the main thread! 
```


* In the example above, we create a thread using the `thread::spawn()` function. The thread loops over `0..5` and print the current value.
* Similarly, we have a main thread where we loop over `0..5` and print the current value.
* We also call `thread::sleep` to force a thread to stop it's execution for a short duration, allowing a different thread to run.
* Notice that we sleep **2** millisecond in the spawned thread and 1 millisecond in the main thread.
* The output from this program might be a little different every time. The important thing to remember here is that if the main thread completes, all threads are shut down whether or not they finished running.
* So, even though the spawned thread should print `i` is 9, it only reaches to **2** because main thread shut down.


### Join Handles in Rust : 

* A spawned threat always return a join handle. If we want the spawned thread to complete execution, we can save the return value of `thread::spawn` in a variable and then call the `join()` method on it.
* The `join` method on `JoinHandle`(return type of `thread::spawn`) waits for the spawned thread to finish.
* Let's look at an example.
  
```rust
use std::thread;
use std::time::Duration;

fn main(){
    // create a thread and save the handle to a variable
    let handel = thread::spawn( || {
        // everything in here runs in separate thread
        for i in 0..10 {
            println!("{} form the spawned thread!", i);
            thread::sleep(Duration::from_millis(2));
        }
    });

    // main thread
    for i in 0..5 {
        println!("{} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // wait for the separate thread to complete
    handel.join().unwrap();
}

```

* Output : 
```plain
0 from the main thread!
0 form the spawned thread!
1 from the main thread!
1 form the spawned thread!
2 from the main thread!
3 from the main thread!
2 form the spawned thread!
4 from the main thread!
3 form the spawned thread!
4 form the spawned thread!
5 form the spawned thread!
6 form the spawned thread!
7 form the spawned thread!
8 form the spawned thread!
9 form the spawned thread!
```

* Here, we save the return of the `thread::spawn()` function and bind it to a variable called `handel`.

* In the final line of the code, we call the `join()` method of th `handel`. Calling `join()` on the `handel` blocks the thread until the thread terminates.
*  The two thread (main and spawned thread) continue alternating for some time, but the main thread waits because of `handel.join()` and does not end until the spawned thread is finished.
*  If we move the `handel.join()` before the final loop, the output will charge and the print statement wont'be interleaved.

```rust
use std::thread;
use std::time::Duration;

fn main(){
    // create a thread and save the handle to a variable
    let handel = thread::spawn( || {
        // everything in here runs in separate thread
        for i in 0..10 {
            println!("{} form the spawned thread!", i);
            thread::sleep(Duration::from_millis(2));
        }
    });

    // wait for the separate thread to complete
    handel.join().unwrap();

    // main thread
    for i in 0..5 {
        println!("{} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

}
```

* Output : 

```plain
0 form the spawned thread!
1 form the spawned thread!
2 form the spawned thread!
3 form the spawned thread!
4 form the spawned thread!
5 form the spawned thread!
6 form the spawned thread!
7 form the spawned thread!
8 form the spawned thread!
9 form the spawned thread!
0 from the main thread!
1 from the main thread!
2 from the main thread!
3 from the main thread!
4 from the main thread!
```
* Thus, it's important to known where `join()` is called. I will dictate whether threads run at the same time or not.

### Using move Closure with Threads in Rust : 

* A value can be moved into a separate thread by passing it as an argument to the `thread::spawn()` function.

```R
use std::thread;

fn main(){
    // min thread starts here
    let message = String::from("Hello, world!");

    // move the message value to a separate thread
    let handel = thread::spawn( move || {
        println!("{}", message);
    });

    // waits for the thread to finish
    handel.join().unwrap();
}
```

* Output : `Hello, world!`

* Here, the closure `||` passed to the `thread::spawn()` function uses the `move` keyword to indicate that it takes ownership of the `message` variable.

* When a value is moved into a thread, ownership of the value is transfer to the thread, and the main thread can no longer access the value.

* This means that the closure can use the `message` variable even after the main thread has completed.

* Let's look at what happens if we don't use the `move` keyword in front of the closure.

```R
use std::thread;

fn main(){
    let message = String::from("Sahilwep");

    // using the message variable without a move
    let handel = thread::spawn( || {
        println!("{}", message);
    });

    handel.join().unwrap();
}
```

* Output : 

```plain
error[E0373]: closure may outlive the current function, but it borrows `message`, which is owned by the current function
 --> src/main.rs:7:33
  |
7 |     let handel = thread::spawn( || {
  |                                 ^^ may outlive borrowed value `message`
8 |         println!("{}", message);
  |                        ------- `message` is borrowed here
  |
```

* The program is this code fails to compile. Here, Rust will try to borrow the `message` variable into the separate thread.

```plain
  |
7 |     let handel = thread::spawn( || {
  |                                 ^^ may outlive borrowed value `message`
```

* However, Rust doesn't know how long the spawned thread will run. Thus it can't tell if the reference to the `message` variable will always be valid.
* By adding the `move` keyword before the closure, we focus the closure to take ownership of the `message` variable or any variable used inside closure.
* We are telling Rust that the main thread won't use the `message` variable anymore. This is a classic example of Rust ownership and how it saves us from mishaps. 
* **Note :** moving a value into a thread can be useful for parallelism, but it can also be a source of bugs if not used carefully.


### Sending Message between Thread in Rust 

* In Rust, threads can communicate with each other by sending message through channels. A channel is a way to send values between threads and it can be used to synchronize communication and avoid data races.
* We can use the `channel()` function in the `std::sync::mspsc` module to create a channel in Rust.
* Let's take a look at how we can use the channels to communicate between threads.

```rust
use std::thread;
use std::sync::mpsc;

fn main(){
    // main thread start here
    // create a new channel
    let (sender, receiver) = mpsc::channel();

    // spawn a new thread
    let handel = thread::spawn( move || {
        // receive message from channel
        let message = receiver.recv().unwrap();

        println!("Received message : {}", message);
    });

    let message = String::from("Sahilwep");
    // send message to channel
    sender.send(message).unwrap();

    // wait for spawned thread to finish
    handel.join().unwrap();
}
``` 
* Output : 
```plain
Received message : Sahilwep
```
* Here, we create a channel using `channel()` function. The `std::sync::mpsc` module provide **multiple-producer, single-consumer** (mspc) channel that can be used to send value between threads.

```rust
// create a new channel
let (sender, receiver) = mspc::channel();
```
* The `sender` and `receiver` variable represent the two endpoints of the channel. The sender endpoint is used to send messages, while the receiver endpoint is used to receive messages.
```rust
// spawn a new thread
let handel = thead::spawn( move || {
    // receive message from channel
    let message = receiver.recv().unwrap();

    println!("Received message : {}", message);
});
```

* We also create a spawned thread using the `thread::spawn()` function. The closure passed to the function received a message using the `receiver.recv()` method.
* The `recv()` method blocks until a message is received on the channel, and it return a `Result` indicating whether a message was received or an error occurred.

```rust
let message = String::from("sahilwep");
// send message to channel
sender.send(message).unwrap();
```
* In the main thread, a `message` is created and send using the `sender.send()` method. The `send()` method result a `Result` indicating whether the message was successfully send or an error occurred.
```rust
// wait for spawned thread to finish
handel.join().unwrap();
```
* Finally, the `join()` method is called on the handle to wait for the spawned thread to finish before the program exits.

* **NOTE :** Here, Another way to handle a `Result` is to use the `unwrap()` method, which will either return the successful value or panic if it's an error. This is useful when we are sure that the function call will always return a successful value and we want to simplify our code.


***
