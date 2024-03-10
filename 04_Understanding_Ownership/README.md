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




