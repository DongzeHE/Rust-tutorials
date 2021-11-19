---
title: 'Rust Course lecture #7'
disqus: hackmd
---
## Table of Contents

[TOC]

This week we will talk about Heap allocation and smart pointers.

# Smart Pointers

## Pointers
A pointer is a general concept for a variable that contains an address in memory. This address refers to, or “points at,” some other data.

We have talked about *reference*, the `&`, in previous lecture, which is just a plain pointer that point at some other data.

Smart pointers, on the other hand, are data structures that not only act like a pointer but also have additional metadata and capabilities. In Rust, the different smart pointers defined in the standard library provide functionality beyond that provided by references. As Rust uses the concept of ownership and borrowing, an additional difference between references and smart pointers is that references are pointers that only **borrow** data; in contrast, in many cases, smart pointers **own** the data they point to.

The types we have seen that allocate data on the heap, `String` and `Vec<T>`, are smart pointers, because they own some memory and allow you to manipulate it. They also have metadata (such as length, capacity) and extra capabilities or guarantees, such as a strings will ensure that  its data will always be valid UTF-8.

Smart points are usually structs with `Deref` and `Drop` traits.
- The `Deref` trait allows an instance of the smart pointer struct to *behave* like a reference so you can write code that works with either references or smart pointers.
- The `Drop` trait allows you to customize the code that is run when an instance of the smart pointer goes out of scope.

We will cover some commonly used smart pointers that are included in `std`. Of course you can implement your own smart pointers, because they are just structs that have specific traits.

## Using Box to Point to Data on the Heap
The most straightforward smart pointer is a box, whose type is written `Box<T>`. Boxes allow you to store data on the heap rather than the stack. What remains on the stack is the pointer to the heap data. 

```rust
fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
}
```

We define the variable `b` to have the value of a `Box` that points to the value `5`, which is allocated **on the heap**. This program will print `b = 5`; in this case, we can *access* the data in the box similar to how we would if this data were *on the stack*. Just like any owned value, when a box goes out of scope, as `b` does at the end of main, it will be deallocated. The deallocation happens for the box (stored on the stack) **and** the data it points to (stored on the heap).

### Why Box?

Transferring ownership of a large amount of data can take a long time because the data is copied around on the stack. To improve performance in this situation, we can store the large amount of data on the heap in a box. Then, only the small amount of pointer data is copied around on the stack, while the data it references stays in one place on the heap. 

### Enabling Recursive Types with Boxes

At compile time, Rust needs to know how much space a type takes up. One type whose size can’t be known at compile time is a recursive type, where a value can have as part of itself another value of the same type. Because this nesting of values could theoretically continue infinitely, Rust doesn’t know how much space a value of a recursive type needs. However, boxes have a known size, so by inserting a box in a recursive type definition, you can have recursive types.

#### the cons list
We will use the cons list, constructed by the `cons` function (short for "construct function"), as the example. The cons function constructs a new pair from its two arguments, which usually are a single value and another pair. These *pairs containing pairs* form a list. More information about the cons list can be found in the [TRPL](https://doc.rust-lang.org/book/ch15-01-box.html#more-information-about-the-cons-list).

Each item in a cons list contains two elements: the value of the current item and the next item. The last item in the list contains only a value called Nil (a custom type, not the "null" we talked about) without a next item. A cons list is produced by recursively calling the cons function. 

The basic structure is like this, but this code cannot compile because the `List` type doesn't have a known size.
```rust
enum List {
    Cons(i32, List),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Cons(2, Cons(3, Nil)));
}
```

If you run the code, rust will tell you the `List` type has infinite size. The compiler starts by looking at the Cons variant, which holds a value of type `i32` and a value of type `List`. Therefore, Cons needs an amount of space equal to the size of an `i32` plus the size of a `List`. To figure out how much memory the List type needs, the compiler looks at the variants, starting with the Cons variant. The Cons variant holds a value of type `i32` and a value of type List, and this process continues infinitely.

![](https://doc.rust-lang.org/book/img/trpl15-01.svg)

#### Using Box to Get a Recursive Type with a Known Size
 
Last week we said that we can pack our type into a Box to fool Rust, because a `Box<T>` is a pointer, Rust always knows how much space a `Box<T>` needs: a pointer’s size doesn’t change based on the amount of data it’s pointing to.

```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
```
The `Cons` variant will need the size of an `i32` plus the space to store the box’s pointer data. The `Nil` variant stores no values, so it needs less space than the `Cons` variant. 

![](https://doc.rust-lang.org/book/img/trpl15-02.svg)

The `Box<T>` type is a smart pointer because it implements the Deref trait, which allows `Box<T>` values to be treated like references. When a `Box<T>` value goes out of scope, the heap data that the box is pointing to is cleaned up as well because of the Drop trait implementation.

## Treating Smart Pointers Like Regular References with the Deref Trait

Implementing the `Deref` trait allows you to customize the behavior of the dereference operator, `*` (as opposed to the multiplication or glob operator). By implementing `Deref` in such a way that a smart pointer can be treated like a regular reference, you can write code that operates on references and use that code with smart pointers too.

A regular reference is a type of pointer, and one way to think of a pointer is as an arrow to a value stored somewhere else.
```rust
fn main() {
    let x = 5;
    let y = &x; // make reference
    let z = Box::new(x); // copy x into a Box


    assert_eq!(5, x);
    assert_eq!(5, *y); // dereference
    assert_eq!(5, *z); // dereference
}
```

### MyBox!
The `Box<T>` type is ultimately defined as a tuple struct with one element. Let's define a `MyBox<T>` type in the same way. 

```rust
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
```

To make `MyBox` work as `Box`, we need to define `Deref` trait.

```rust
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T; // define an alias

    fn deref(&self) -> &Self::Target {
        &self.0 // return a reference to the value
    }
}
```

Without the `Deref` trait, the compiler can only dereference `&` references. The `deref` method gives the compiler the ability to take a value of any type that implements `Deref` and call the `deref` method to get a `&` reference (b/c ownership!) that it knows how to dereference. When calling `*y`, Rust actually runs `*(y.deref())`

### Implicit Deref Coercions with Functions and Methods

Deref coercion converts a type that implement the `Deref` trait into a reference to another type. **It happens automatically.** For example, deref coercion can convert `&String` to `&str` because String implements the `Deref` trait such that it returns `&str`. 

```rust=
fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]);
    hello(&m); // &Box -> &String -> &str
}
```

## How Deref Coercion Interacts with Mutability
Similar to how you use the `Deref` trait to override the `*` operator on immutable references, you can use the `DerefMut` trait to override the `*` operator on *mutable* references.

Rust does deref coercion when it finds types and trait implementations in three cases:
* From `&T` to `&U` when `T: Deref<Target=U>`
* From `&mut T` to `&mut U` when `T: DerefMut<Target=U>`
* From `&mut T` to `&U` when `T: Deref<Target=U>`

Rust will not coerce a immutable reference to a mutable one, because of the borrowing rules. 

## Running Code on Cleanup with the Drop Trait

`Drop` trait requires you to implement one method named `drop` that takes a mutable reference to `self`. It lets you customize what happens when a value is about to go out of scope. You can provide an implementation for the `Drop` trait on **any type**. It is crucial for smart pointer b/c those pointers also need to take care of the memory chunk on the heap.

The Drop trait requires you to implement one method named `drop` that takes a mutable reference to self.

```rust
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
}
```

Rust automatically called `drop` for us when our instances went out of scope, calling the code we specified. Variables are dropped in the reverse order of their creation, so `d` was dropped before `c`. This example gives you a visual guide to how the drop method works; usually you would specify the cleanup code that your type needs to run rather than a print message.

You can also call the `std::mem::drop` function explicitly to drop a value early. You cannot call the `drop` method of the `Drop` trait manually because Rust will call it when a value goes out of the scope.

```rust
fn main() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}
```

You also don’t have to worry about problems resulting from accidentally cleaning up values still in use: the ownership system that makes sure references are always valid also ensures that drop gets called only once when the value is no longer being used.

## other smart pointers

### Rc, the Reference Counted Smart Pointer

In the majority of cases, ownership is clear: you know exactly which variable owns a given value. However, there are cases when a single value might have multiple owners. For example, in graph data structures, multiple edges might point to the same node, and that node is conceptually owned by all of the edges that point to it. A node shouldn’t be cleaned up unless it doesn’t have any edges pointing to it.

To enable multiple ownership, Rust has a type called `Rc<T>`, which is an abbreviation for reference counting. The `Rc<T>` type keeps track of the number of references to a value to determine whether or not the value is still in use. If there are zero references to a value, the value can be cleaned up without any references becoming invalid.

### When to use?

We use the `Rc<T>` type when we want to allocate some data on the heap for multiple parts of our program to read and we can’t determine at compile time which part will finish using the data last. If we knew which part would finish last, we could just make that part the data’s owner, and the normal ownership rules enforced at compile time would take effect. Note that `Rc<T>` is only for use in **single-threaded** scenarios.

### a fancy cons list
We’ll create two lists that both share ownership of a third list.  We’ll create list a that contains `5` and then `10`. Then we’ll make two more lists: `b` that starts with `3` and `c` that starts with `4`. Both `b` and `c` lists will then continue on to the first a list containing `5` and `10`. In other words, both lists will share the first list containing `5` and `10`.
![](https://doc.rust-lang.org/book/img/trpl15-03.svg)

If we use Box to do this, Rust will complain because Box needs to take ownership of its value.

```rust
enum List <'a> {
    Cons(i32, Box<&'a List<'a>>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let nil = Nil;
    let a = Cons(10, Box::new(&nil);
    let b = Cons(5, Box::new(&a)));
    let c = Cons(3, Box::new(&b));
    let d = Cons(4, Box::new(&b)); 
}
```

We could change the definition of `Cons` to hold references instead, but then we would have to specify lifetime parameters. By specifying lifetime parameters, we would be specifying that every element in the list will live at least as long as the entire list. The borrow checker wouldn’t let us compile `let a = Cons(10, &Nil);` for example, because the temporary `Nil` value would be dropped before `a` could take a reference to it.

```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}

```

A better solution is to use `Rc<T>`. The call to `Rc::clone` only increments the reference count, which doesn’t take much time.

```rust
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a)); // Fully qualified syntax, preferred
    let c = Cons(4, a.clone()); // Method-call syntax
}
```

To check the reference count, let's do some complicated stuff.

```rust
fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    } // Drop decreases the reference count automatically
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
```

Via immutable references, `Rc<T>` allows you to share data between multiple parts of your program for **reading only**.

# Heap Allocation

# Why heap?





