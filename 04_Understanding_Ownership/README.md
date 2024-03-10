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


