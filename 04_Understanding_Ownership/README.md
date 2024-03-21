# Ownership in Rust:

* Ownership is a set of rule that governs how a rust program manages memory. 

* Rust uses memory management through a system of ownership with a set of rules that compiler checks. if any of the rules are violated, teh program won't compile.

* Rust ownership is new concept for many programmers, it does take some time to get used to. The good news is that the more experienced you become with rust and the rules of the ownership system, the easier you'll find it to naturally develop code that is safe and efficient. 

## Stack & Heap: 

* System programming language like Rust, whether a value is on the stack or the heap affect how the language behaves and why we have to make certain decisions.

* Both `stack` and `heap` are parts of memory available to our code to use at run time, but they structured in different ways.
  * `Stack` : follows `LIFO`
  * `Heap` : follows `FIFO`


* `Stacks` stores values in order it gets them and removes the values in the opposite order. This is referred as `LIFO`. All data stored on the stack must have a known, fixed size.

* `Heap` is less organized: when we put data on the heap, we request a certain amount of space. The memory allocator finds an empty spot in the heap that is big enough, marks it as being use and return a `pointer`, which is the address of that location. This process is called allocating on the heap, or sometimes abbreviated as just allocating(pushing value onto the stack is not considered allocating). `because pointer to the heap is known, fixed size, we can store the pointer on the stack, but when we want the actual data, we must follows the pointer` Example: Think of being seated at restaurant, when we enter, we state the number of people in our group, and the host finds an empty table that fits everyone and leads us there. if someone in our group comes late, they can ask where we have been seated to find us.

* `Pushing to the stack is faster than allocating on the heap because allocator never has to search for an place to store new data; that location is always top on the stack` Comparatively, `allocating space on the heap requires more work because the allocator must first find a big enough space to hold the data and then perform bookkeeping to prepare for the next allocation`.

* `Accessing data in the heap is slower than accessing data on the stack because we have to follow a pointer go get there`. contemporary processors are faster if they jump around less in memory. Continuing the analogy, consider a server at a restaurant taking orders from many tables. It's most efficient to get all the order at one table before moving on the next table. Taking an order from table A, then an order from table B, then one from A again, and then from B again would be a much slower process. By the same token, a processor can do its job better if it works on data that's close to other data(as it is on the stack) rather than farther away(as it can be on the heap).

* `When our code calls a function, the values passed into the function(including, potentially, pointer(i.e on stack) to data(i.e on heap) on the heap) and the function's local variable get pushed onto the stack. when the function is over, those value get popped off the stack`.

* Keeping track of what parts of code are using what data on the heap, minimizing the amount of duplicate data on the heap, and cleaning up unused data on the heap so we don't run out of space are all problems that ownership addresses. Once we understand ownership, we won't need to think about the stack and heap very often, but knowing that the main purpose of ownership is to manage heap can help explain why it works the way it does.

## Ownership Rules: 

* Keep these rule in mind as we work through the examples that illustrate them:
  * `Each value in Rust has an owner`
  * `There can only be one owner at a time`
  * `When the owner goes out of scope, the value will be dropped`.

## Variable Scope:

* Scope is the range within a program for which an item is valid. 

```rust
{       // s is not valid here, it's not yet declared
    let s = "hello";    // s is valid from this point forward
    // do stuff with s
}       // this scope is now over, and s is no longer valid
```
* Points to remember here, 
  * When `s` comes into scope, it is valid.
  * It remains valid until it goes out of scope.


***
### String Literals & String Object

#### **String Literals**

* String literals (`&str`) are used when the value of a string is `known` at compile time. String literals are a set of characters, which are hardcoded into a variable for example, `let name = "Sahil"`. String literals are found in module `std::str`. String literals are also known as string slices.

```rust
fn main() {
    let name: &str = "sahil";
    let username:&'static str = "sahilwep"; // this way we can also declare string literals.
}
```
* String literals are static and stored in the program's binary.
* String literals are stored in the program's binary and typically reside in the read-only memory section. This memory is part of the executable file and is separate from the stack and heap used during runtime.

#### **String Objects**

* String object type is provided in Standard library. Unlike string literal, the string object type is not a part of core language. It is defined as public structure in standard library pub struct String. String is growable collection.
* String are dynamic and stored in the heap.
* They have a variable size and can grow or shrink at runtime.
* Strings are mutable, and can modify their contents.
* They are the type `String`

```rust
fn main() {
    let mut name = String::from("Sahil");
    let mut username = String::new();
}
```
* (`String` objects), which are allocated on the heap at runtime and can change in size.

* In Summary, String literals are compile-time constants with fixed size, while strings(String objects) are dynamic, heap-allocated data structure that can be modified during runtime.

***
## The String Type

* We want to look at data that is stored on the heap and explore how Rust knows when to clean up that data, and the `String` type is great example.

* We've already seen String literals, where a string value is hardcoded into our program. String literals are convenient, but they aren't suitable for every situation in which we may want to use text. One reason is that they're immutable. Another is that not every string value can be known when we write our code: For example, what if we wan't to take user input and store? For these situations, Rust has a second string type, `String`. This type manages data allocated on the heap and as such is able to store an amount of text that is unknown to us at compile time. You can create a `String` from string literal using the `form` function, like so:

```rust
let s = String::from("hello");
```

* The double colon `::` operator allows us to namespace this particular `from` function under the `String` type rather than using some sort of name like `string_from`. 

* This kind of string can be mutated:
```rust
let mut s = String::from("Hello");
s.push_str(", world!");     // push_str() appends a literal to a String
println!("{}", s);  // This will print `hello world!`
```
* So , what's the difference here? Why can `String` be mutated but literals cannot? The difference is in how these two types deal with memory.


### Memory and allocation:

* In case of a String literal, we known the contents at compile time, so the text is hardcoded directly into the final executable. This is why string literals are fast and efficient. But these properties only come from the string literal's immutability. 

* With the `String` type in order to support a mutable, growable piece of text, we need to allocate an amount of memory on the heap, unknown at compile time and hold the contents. This means:
  * The memory must be requested from the memory allocator at runtime.
  * We need a way of returning this memory to the allocator when we're done with our `String`.

* That first part is done by when we call `String::from`, its implementation requests the memory it needs. This is pretty much universal in programming language.
* However, the second part is different. In language with a *garbage collector*(GC), the GC keeps track of and cleans up memory that isn't being used anymore, and we don't need to think about it. In most language without a GC, It;s our responsibility to identify when memory is no longer being used and to call code to explicitly free it, just as we did to request it. Dong this correctly has historically been a difficult programming problem. If we forgot, we'll wast memory. If we do it too early, we'll have an invalid variable. If we do it twice, that's bug too. We need to pair exactly one `allocate` and exactly one `free`.

* `Rust takes a different path: the memory is automatically returned once the variable that owns it goes out of scope`. Example using a `String` instead of a string literal:

```rust
{
    let s = String::from("hello");      // s is valid from this point forward

    // do stuff with s
}   // this scope is now over, ans s is no longer valid.

```

* There is a natural point at which we can return the memory our `String` needs to the allocator: when `s` goes out of scope. When a variable goes out out of scope, Rust calls a special function for us. This function is called `drop`, and it's where the author of `String` can put the code to return the memory. Rust calls `drop` automatically at the closing curly bracket.

* **NOTE:** In C++, this pattern of deallocating resources at the end of an item's lifetime is sometimes called *Resource Acquisition Is Initialization (RAII)*. The `drop` function in Rust will be familiar to you if you've used RAII patterns.

* This pattern has a profound impact on the way Rust code is written. It may seems simple right now, but the behavior of code can be unexpected in more complicated situations when we want to have multiple variable use that data we've allocate on the heap. Let's explore some of those situations now.

### Variables and Data Interacting with move:
* Multiple variable can interact with the same data in different ways in Rust. let's look at an example using an integer: 

```rust
let x = 5;
let y = x;
```

* We bind the value `5` to `x`; then  make a copy of the value in `x` and bind it to `y`. We now have two variables, `x` and `y`, and both are equal `5`. This is indeed what is happening, because integers are simple values with known, fixed size, and these two `5` value are pushed onto the stack.

* Now let's look at the `String` version:
```rust
let s1 = String::from("hello");
let s2 = s1;
```

* This looks very similar, so we might assume that the way it works be the same: that is, the second line would make a copy of the value in `s1` and bind it to `s2`. But this isn't quite what happens.

* Take a look at figure 4.1, A `String` is made up of three parts, shown on the left: a pointer to the memory that holds the contents of the string, a length and a capacity. This group of data is stored on the stack, On the right is the memory on the heap that holds the contents.
  
* Fig 4.1 ![image](/04_Understanding_Ownership/assets/img1.png)

* The length is how much memory, in bytes, the contents of the `String` are currently using. The capacity is the total amount of memory, in bytes, that the `String` has received from the allocator. The difference between length and capacity matters, but not in this context, so for now, it's fine to ignore the capacity.

* When we assign `s1` to `s2`, the `String` data is copied, meaning we copied the pointer, the length, and capacity that are on the stack we do not copy the data on the heap that the pointer refers to. In other words, the data representation in memory looks like figure 4.2:

* fig: 4.2 ![image](/04_Understanding_Ownership/assets/img2.png)

* The representation does not look like in below figure 4.3, which is what memory would look like if Rust instead copied the heap data as well. If Rust did this, the operation `s2 = s1` could be very expensive in terms of runtime performance if the data on the heap were large.

* fig 4.3  ![image](/04_Understanding_Ownership/assets/img3.png)

* Earlier, we said that when variable goes out of scope, Rust automatically calls the `drop` function and cleans up the heap memory for that variable. But in fig 4.2 shows both data pointer pointing to the same location. This is a problem: when `s2` and `s1` go out of scope, they will both try to free the same memory. This is known as *double free* error and is one of the memory safety bugs we mentioned previously. Freeing memory twice can lead to memory corruption, which can potentially lead to security vulnerabilities.

* To ensure memory safety, after the line `let s2 = s1`, Rust considers `s1` as no longer valid. Therefore, Rust doesn't need to free anything when `s1` goes out of scope. Check out what happens when you try to use `s1` after `s2` is created: it won't work:

```rust
let s1 = String::from("hello");
let s2 = s1;

println!("{}", s1);
```
* You'll get an error like this because Rust prevents you from using the invalidate reference:

```sh
$ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0382]: borrow of moved value: `s1`
 --> src/main.rs:5:28
  |
2 |     let s1 = String::from("hello");
  |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
3 |     let s2 = s1;
  |              -- value moved here
4 |
5 |     println!("{}, world!", s1);
  |                            ^^ value borrowed here after move
  |
  = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider cloning the value if the performance cost is acceptable
  |
3 |     let s2 = s1.clone();
  |                ++++++++

For more information about this error, try `rustc --explain E0382`.
error: could not compile `ownership` due to previous error
```

* If you've heard the term *shallow copy* and *deep copy* while working with other languages, the concept of copying the pointer, length, and capacity without copying the data probably sounds like making a shallow copy. But because Rust also invalidates the first variable, instead of being called a shallow copy, it's known as a ***move***. In this example, we would say that `s1` was *moved* into `s2`. So what actually happens is shown in figure 4.4

* Fig 4.4 ![image](/04_Understanding_Ownership/assets/img4.png)

* This solves our problem! With only `s2` valid, when it goes out of scope it alone will free the memory, and we're done.

* In addition, there's a design choice that's implemented by this: Rust will never automatically create "deep" copies of your data. Therefore, any *automatic* copying can be assumed to be inexpensive in terms of runtime performance.


### Variable and Data interacting with Clone

* If we do want to deeply copy the heap of the `String`, not just the stack data, we can use a common method called `clone`. We'll discuss method syntax later on, but because methods are a common features in many programming languages, you've probably seen them before. Example of the `clone()` method in action: 

```rust
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
```

* This works just fine and explicitly produces the behavior shown in figure 4.3, where the heap data gets copied.

* When you see the `clone` you known that some arbitrary code is being executed and that code may be expensive. it's a visual indicator that something different is going on.

### Stack-Only Data: Copy

* There's another wrinkle we haven't talked about yet. This code using integer-part of which was shown in Listing 4.2-works and is valid:

```rust
let x = 5;
let y = x;

println!("x = {}, y = {}", x, y);
```

* But this code seems to contradict what we just learned: we don't have to call to `clone`, but `x` is still valid and wasn't moved into `y`.

* This reason is that types such as integers that have a known size at compile time are stored entirely on the stack, so copies of the actual values are quick to make. Thats means there's no reason we would want to prevent `x` from being valid after we create the variable `y`. In other words, there's no difference between deep and shallow copying here, so calling `clone` wouldn't do anything different from the usual shallow copying, and we can leave it out.

* Rust has a special annotation called the `Copy` trait that we can place on types that are stored on the stack, as integers are. If a type implements the `Copy` trait, variable that use it do not move, but rather are trivially copied, making them still valid after assigning to another variable.

* Rust won't let us annotate a type with `Copy` if the type, or any of its parts, has implemented the `Drop` trait. If the type needs something special to happen when the value goes out of scope and we add the `Copy` annotation to that type, we'll get a compile-time error.

* So, what type implement the `Copy` trait? You can check the documentation for the given type to be sure, but as a general rule, any group of simple scalar value can implement `Copy`. Here are some of the type that implement `Copy`:
  * All the integer types, such as `u32`
  * The Boolean type, `bool`, with values `true` and `false`.
  * All the floating-point type, such as `f64`.
  * The character type, `char`.
  * Tuples, if they only contain type that also implement `Copy`. For example, `(i32, i32) ` implement `Copy` but `(i32, String)` does not.


### Ownership and Functions

* The mechanics of passing a values to a functions are similar to those when assigning a value to a variable. Passing a variable to a function will move or copy, just as assignment does.
* Example: 

```rust
fn main() {
    let s = String::from("hello");  // s comes into scope
    takes_ownership(s);     // s values moved into the function..., and so is no longer valid here.

    let x = 5;      // x comes into scope

    makes_copy(x);      // x would move into the function, but 'i32' is Copy, so it's okay to still use x afterward

}   // here, x goes out of scope, then s. But because s's value was moved, nothing special happens.


fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}   // here, some_string goes out of scope and `drop` is called. The backing memory is freed.


fn makes_copy(some_integer: i32) {  // some_integer comes into scope
    println!("{}", some_integer);
}   // here, some_integer goes out of scope. 
```

* If we tried to use `s` after the call to `takes_ownership`, Rust would throw a compile-time error. These static checks protects us from mistakes. Try adding code to `main` that uses `s` and `x` to see where you can use them and where the ownership rules prevent you from doing so.


### Return Values and Scope

* Returning values can also transfer ownership. below shown an example of function that returns some value, with similar annotations as those in above example.

```rust
fn main() {
    let s1 = gives_ownership();     // gives_ownership moves its return value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);      // s2 is moved into takes_and_give_back, which also moves its return value into s3 

}   // here, s3 goes out of scope and is dropped. s2 was moved, so nothing happens. s1 goes out of scope and is dropped.


fn gives_ownership() -> String {    // gives_ownership will move its return value into the function that calls it

    let some_string = String::from("yours");    // some_string comes into scope

    some_string // some_string is returned and moves out to the calling function
}

fn takes_and_gives_back(a_String: String) -> String {       // some_string comes into scope
    a_String        // some_string is returned and moves out to the calling function
}
```

* The ownership of a variable follows the same pattern every time: assigning a value to another variable moves it. When a variable that includes data on the heap goes out of scope, the value will be cleaned up by `drop` unless ownership of the data has been moved to another variable.

* While this works, taking ownership and then returning ownership with every function is a bit tedious. What if we want to let a function use a value but not take ownership? It's quite annoying that anything we pass in also needs to be passed back if we want to use it again, in addition to any data resulting from the body of the function that we might want to return as well.

* Rust does let us return multiple value using a tuple, as shown in below example:

```rust
fn main() {
    let s1 = String::from("hello");
    
    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is '{}'", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();   // len() returns the length of a String

    (s, length)
}
```
* But this is too much ceremony and a lot of work for a concept that should be common. Luckily for us, Rust has a feature for using a value without transferring ownership, called *references*


## References and Borrowing

* The issue with the tuple code in listing above is that we have to return the `String` to the calling function so we can still use the `String` after the call to `calculate_length`, because the `String` was moved into `calculate_length`. Instead, we can provide a reference to the `String` value. A *reference* is like a pointer in that it's an address we cna follow to access the data stored at the address; that data is owned by some other variable. Unlike a pointer, a reference is guaranteed to point a valid value of a particular type for the life of that reference.

* Here is how you would define and use a `calculate_length` function that has a reference to an object as a parameter instead of taking ownership of the value: 

```rust
fn main() {
    let s1 = String::from("hello");
    
    let len  = calculate_length(&s1);

    println!("The length of '{}' is '{}'", s1, len);
}

fn calculate_length(s: &String) -> (usize) {
    s.len()  // return the length of string.
}
```

* First, notice that all the tuple code in the variable declaration and the function return value is gone. Second, notice that we pass `&s1` into `calculate_length` and, in it's definition, we take `&String` rather than `String`. These ampersand represent *references*, and they allows you to refer to some value without taking ownership of it.
* ![image](/04_Understanding_Ownership/assets/img5.png)

* **NOTE:** The opposite of referencing by using `&` is dereferencing, which is accomplished with the dereference operator, `*`. We'll see some uses of the dereference operator in later on chapters.

* Let's take a closer look at the function call here: 

```rust
let s1 = String::from("hello");
let len = calculate_length(&s1);
```

* The `&s1` syntax let us create a reference that *refers* to the value of `s1` but does not own it. Because it does ont own it, the value it points to will not be dropped when the reference stops being used.
* Likewise, the signature of the function uses `&` to indicate that the type of parameter `s` is reference. Let's add some explanatory annotations:

```rust
fn calculate_length(s: &String) -> usize {
  s.len()
}
```
* The scope in which the variable `s` is valid is the same as any function parameter's scope, because the value pointed to by reference is not dropped when `s` stops being used, because `s` doesn't have ownership. When functions have references as parameters instead of the actual value, we won't need to return the values in order to give back ownership, because we never had ownership.

* We call the action of creating a reference *borrowing*. As in real life. if a person owns something, you can borrow it from them. When you're done, you have to give it back. You don't own it.

* Let's see what happens if we try to modify something we're borrowing?
```rust
fn main() {
    let s1 = String::from("hello");
    
    change(&s1);
}

fn change(s: &String) {
    s.push_str(", world");
}
```
* Output : gives error 
```sh
$ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0596]: cannot borrow `*some_string` as mutable, as it is behind a `&` reference
 --> src/main.rs:8:5
  |
7 | fn change(some_string: &String) {
  |                        ------- help: consider changing this to be a mutable reference: `&mut String`
8 |     some_string.push_str(", world");
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable

For more information about this error, try `rustc --explain E0596`.
error: could not compile `ownership` due to previous error
```
* Just as variable are immutable by default, so are references. We're not allowed to modify something we have a reference to.

### Mutable Reference: 

* We can change the reference value by using `mut` keyword during variable defining, and `&mut` keyword during the function parameter definition, and we can use `&mut` during the function calling.

```rust
fn main() {
    let mut s1 = String::from("hello");
    
    change(&mut s1);
    println!("value inside the string is : {}", s1);
}

fn change(s: &mut String) {
    s.push_str(", world");
}
```

* **Mutable reference have one big restriction:** if you have a mutable reference to a value, you can have no other references to that value. Example below: 

```rust
fn main() {
    let mut s1 = String::from("hello");
    
    let r1 = &mut s1;
    let r2 = &mut s1;

    println!("{}, {}", r1, r2);
}
```
* Output : Error 

```sh
$ cargo run
error[E0499]: cannot borrow `s1` as mutable more than once at a time
 --> main.rs:5:14
  |
4 |     let r1 = &mut s1;
  |              ------- first mutable borrow occurs here
5 |     let r2 = &mut s1;
  |              ^^^^^^^ second mutable borrow occurs here
6 |
7 |     println!("{}, {}", r1, r2);
  |                        -- first borrow later used here

error: aborting due to previous error

For more information about this error, try `rustc --explain E0499`.
```

* **This error says that this code is invalid because we cannot borrow `s` as mutable more than once at a time**. The first mutable borrows is in `r1` and must last until it's used in the `println!`, but between the creation of that mutable reference and its usage, we tried to create another mutable reference in `r2` that borrows the same data as `r1`.

* The restriction preventing multiple mutable references to the same data at the same time allows for mutation but in very controlled fashion.It's something that new Rustacens struggle with because most language let you mutate whenever you'd like. The benefit of having this restriction is that rust can prevent data races at compile time. A *data races* is similar to a race condition and happens when these three behaviors occurs:
  * Two or more pointers access the same data at the same time.
  * At least one of the pointer is being used to write to the data.
  * There's no mechanism being used to synchronize access to the data.
* Data races cause undefined behavior and can be difficult to diagnose and fix when you're trying to track them down at runtime; Rust prevents this problem by refusing to compile code with data races!

* As races cause undefined behavior adn can be difficult to diagnose and fix when you're trying to track them down at runtime; Rust prevents this problem by refusing to compile code with data races!

* As always, we can use curly brackets to create a new scope, allowing for multiple mutable references, just not simultaneous ones:

```rust
fn main() {
    let mut s1 = String::from("hello");
    
    {
        let r1 = &mut s1;
    }   // r1 goes out of scope here, so we can make new reference with no problems.

    let r2 = &mut s1;
}
```

* Rust enforces a similar rule for combining mutable and immutable references. This code results in an error:

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s;  // no problem
    let r2 = &s;  // no problem
    let r3 = &mut s;  // BIG PROBLEM

    println!("{}, {}, {}", r1, r2, r3);
}
```
* Output : Error

```sh
$ cargo run
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
 --> main.rs:6:14
  |
4 |     let r1 = &s;  // no problem
  |              -- immutable borrow occurs here
5 |     let r2 = &s;  // no problem
6 |     let r3 = &mut s;  // BIG PROBLEM
  |              ^^^^^^ mutable borrow occurs here
7 |
8 |     println!("{}, {}, {}", r1, r2, r3);
  |                            -- immutable borrow later used here

error: aborting due to previous error

For more information about this error, try `rustc --explain E0502`.
```

* Whew! we also cannot have a mutable reference while we have an immutable one to the same value.

* User of an immutable reference don't expect the value to suddenly change out from under them! However, immutable reference are allowed because no one who is just reading the data has the ability to affect anyone else's reading of the data.

* Note that a reference's scope starts from where it is introduced and continues through the last time that reference is used. For instance, this code will compile because the last usage of the immutable references, the `println!`, occurs before the mutable references is introduced:

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s;  // no problem
    let r2 = &s;  // no problem
    
    println!("{}, {}", r1, r2); 
    
    // variable r1 and r2 will be used after this point
    let r3 = &mut s;  // no problem
    println!("{}", r3);
}
```

* The scopes of the immutable references `r1` and `r2` end after the `println!` where they are last used, which is before the mutable reference `r3` is created. These scopes don't overlap, so this code is allowed: the compiler can tell that the reference is no longer being used at a point before the end of the scope.

* Even though borrowing errors may be frustrating at times, remember that it's the Rust compiler pointing out a potential bug early(at compile time rather than at runtime) are showing you exactly where the problem is. Then you don't have to track done why your data isn't what you thought it was.

### Dangling References: 

