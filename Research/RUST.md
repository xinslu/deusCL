\# Introduction

My choice for building the REPL for this project would be Rust. Now, I do admit this choice is baised, I wanted to get into learning rust because I hope to expand myself into WASM and Systems Engineering among many other things and rust was my choice over C++ other its obvious unneeded complexity and annoying errors.  This document will summarie what I find about rust and a way to summarize all that I think about it.

# Rust

Rust, for now, is supposedly the most loved language above even Python. Above all, for this project I think I value it's speed and exectuables more than the others. If I need to run a Common Lisp REPL, I would defintely expect it to be fast and accurate. With Rust, I can achieve that while at the same time having a fairly high-level syntax.

# Garbage Collection

Garbage Collection just does not exsist in Rust. So, from what I've learned programming languages have 3 ways of dealing with memory allocation.
* Implicit Garbage Collection when the memory spot is no longer used
* Explicit Declaration of Memory at creation and deallocation.
* A system of Ownership and Borrowing like Rust does with all variables defined in a functional scope.

# Stacks and Heaps

Before I start with Ownership and Borrowing, I need to understand just how pointers work and for that I need to understanding the execution cycle and memory allocation in terms of stacks and heaps. Like most languages Rust keeps track of RunTime execution in the stack, but along with that it also stores all the variables that have a fixed size like an unsigned/signed int or a tuple to name a few. The data that required a size that is unknown at compile time must be stored in the heap. This takes time because the heap must find a place on the heap that matches the size of the data. So, storing on the heap is defintely slower. After the data is stored on the heap, a pointer reference is returned to the execution stack in consideration here. Keeping track of what parts of code are using what data on the heap, minimizing the amount of duplicate data on the heap, and cleaning up unused data on the heap so you don’t run out of space are all problems that ownership addresses. With that in mind let's move up a level of abstraction to Ownership and Borrowing.

# Ownership
There are 3 rules to the Rust Ownership:
* Each value in Rust has a variable that’s called its owner.
* There can only be 1 owner at a time for a variable.
* When the owner goes out of scope, the value will be dropped.
By default, all the variables in Rust are defined in a block scope. So outside, a block a variable will not be accesible and it gets deallocated as execution finishes the block. This is done by a special function in rust called drop. Now let's get into some details about ownership. An example I came across, was:

```
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}, world!", s1);
}
```
Instead of copying the values from the s1 pointer and allocating more memory in the heap to store s2, which to say the least would be computationally expensive and slow, rust instead transfer ownership from s1 onto s2, which essentially means the pointer store is moved onto s2 and s1 is invalidated. This is essentially a form of shallow copy but since s1 is also invalidated, rust calls this a "move". A deep copy can be created if wanted but for heap stored variables its never explicit. On Stack-Only datatypes, this is implicit since they have a Copy trait.

# Borrowing
There is a hidden problem in ownership, passing a variable of heap-memory into a fucntion would always invalidate it since the paramater would take up its value, which would automatically would be invalidated after the function block. So for each heap-memory type variable we would have to return the variable in the function to preserve the value. This is solved by borrowing, which essentially means passing the reference of the variable instead of the transferring the ownership. However, like normal variables references are immutable and must be declared to be mutable throught &mut. Mutable references have one big restriction: you can have only one mutable reference to a particular piece of data at a time. This applied to having different mutable and immutable references too. To summarize the rules:
* There should be only eithr one mutable reference or any number of immutable references.
* References should always be made from a variable that is valid.

There is also another type of Refrence that is called slice reference that makes use of the range operator. The slice reference ensure that the data it is dependent on is not deferenced or deallocated. This ensures memory safety in Rust.


