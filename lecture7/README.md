# Heap allocation and Smart Pointers

Source: [The Rust Book](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)


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


## Box

The most straightforward smart pointer is a box. Boxes allow you to store data on the heap rather than the stack. What remains on the stack is the pointer to the heap data. You’ll use them most often in these situations:

 - When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size
 - When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so
 - When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type

### Using a Box to Store Data on the Heap

```rust
fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
}
```

We define the variable b to have the value of a Box that points to the value 5, which is allocated on the heap. This program will print b = 5; in this case, we can access the data in the box similar to how we would if this data were on the stack. Just like any owned value, when a box goes out of scope, as b does at the end of main, it will be deallocated. The deallocation happens for the box (stored on the stack) and the data it points to (stored on the heap).

Boxed values can be dereferenced using the `*` operator, which removes one layer of indirection. Below we have a more complicated example, which we will talk through line by line. 

```rust
use std::mem;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

// A Rectangle can be specified by where its top left and bottom right 
// corners are in space
#[allow(dead_code)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}

fn boxed_origin() -> Box<Point> {
    // Allocate this point on the heap, and return a pointer to it
    Box::new(Point { x: 0.0, y: 0.0 })
}

fn main() {
    // (all the type annotations are superfluous)
    // Stack allocated variables
    let point: Point = origin();
    let rectangle: Rectangle = Rectangle {
        top_left: origin(),
        bottom_right: Point { x: 3.0, y: -4.0 }
    };

    // Heap allocated rectangle
    let boxed_rectangle: Box<Rectangle> = Box::new(Rectangle {
        top_left: origin(),
        bottom_right: Point { x: 3.0, y: -4.0 },
    });

    // The output of functions can be boxed
    let boxed_point: Box<Point> = Box::new(origin());

    // Double indirection
    let box_in_a_box: Box<Box<Point>> = Box::new(boxed_origin());

    println!("Point occupies {} bytes on the stack",
             mem::size_of_val(&point));
    println!("Rectangle occupies {} bytes on the stack",
             mem::size_of_val(&rectangle));

    // box size == pointer size
    println!("Boxed point occupies {} bytes on the stack",
             mem::size_of_val(&boxed_point));
    println!("Boxed rectangle occupies {} bytes on the stack",
             mem::size_of_val(&boxed_rectangle));
    println!("Boxed box occupies {} bytes on the stack",
             mem::size_of_val(&box_in_a_box));

    // Copy the data contained in `boxed_point` into `unboxed_point`
    let unboxed_point: Point = *boxed_point;
    println!("Unboxed point occupies {} bytes on the stack",
             mem::size_of_val(&unboxed_point));
}
```

### Recursive Types with Box

**At compile time, Rust needs to know how much space a type takes up.** One type whose size can’t be known at compile time is a recursive type, where a value can have as **part of itself another value of the same type**. Because this nesting of values could theoretically continue infinitely, Rust doesn’t know how much space a value of a recursive type needs. However, boxes have a known size, so by inserting a box in a recursive type definition, you can have recursive types.

We can implement this using a Cons list. **A cons list** is a data structure that comes from the Lisp programming language and its dialects. In Lisp, the cons function (short for “construct function”) constructs a new pair from its two arguments, which usually are a single value and another pair. These pairs containing pairs form a list.

The cons function concept has made its way into more general functional programming jargon: “to cons x onto y” informally means to construct a new container instance by putting the element x at the start of this new container, followed by the container y.

Each item in a cons list contains two elements: the value of the current item and the next item. The last item in the list contains only a value called Nil without a next item. **A cons list is produced by recursively calling the cons function.** The canonical name to denote the base case of the recursion is Nil. 

To implement this data structure, we can first define the following enum.

```rust
enum List {
    Cons(i32, List),
    Nil,
}
```

Then we can create our list as shown below.

```rust
use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Cons(2, Cons(3, Nil)));
}
```

When compiling this, we get an error. Why? Rust can’t figure out how much space to allocate for recursively defined types.

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

**Boxes provide only the indirection and heap allocation**; they don’t have any other special capabilities, like those we’ll see with the other smart pointer types. They also don’t have any performance overhead that these special capabilities incur, so they can be useful in cases like the cons list where the indirection is the only feature we need.


### The Deref trait

Implementing **the Deref trait allows you to customize the behavior of the dereference operator, * (as opposed to the multiplication). By implementing Deref in such a way that a smart pointer can be treated like a regular reference, you can write code that operates on references and use that code with smart pointers too.

Here is a simple example of using Box like a reference.

```rust
fn main() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

The trait that enables this behavior is known as the `Deref` trait. We can see how it works with the following example.

```rust
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
```

Implementing the `Deref` trait allows you to customize the behavior of the dereference operator, `*` (as opposed to the multiplication or glob operator). By implementing `Deref` in such a way that a smart pointer can be treated like a regular reference, you can write code that operates on references and use that code with smart pointers too.

A regular reference is a type of pointer, and one way to think of a pointer is as an arrow to a value stored somewhere else.

```rust
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
```

This would allow us to compile the following example.

```rust
fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

Behind the scenes in this example, what is actually run is this.

```rust
*(y.deref())
```

Without the `Deref` trait, the compiler can only dereference `&` references. The `deref` method gives the compiler the ability to take a value of any type that implements `Deref` and call the `deref` method to get a `&` reference (b/c ownership!) that it knows how to dereference. When calling `*y`, Rust actually runs `*(y.deref())`


### The Drop trait

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

### Arc

Similar to `Rc`, `Arc` (atomic reference counted) can be used when sharing data across threads. This struct, via the `Clone` implementation can create a reference pointer for the location of a value in the memory heap while increasing the reference counter. As it shares ownership between threads, when the last reference pointer to a value is out of scope, the variable is dropped.

```rust

fn main() {
    use std::sync::Arc;
    use std::thread;

    // This variable declaration is where its value is specified.
    let apple = Arc::new("the same apple");

    for _ in 0..10 {
        // Here there is no value specification as it is a pointer to a reference
        // in the memory heap.
        let apple = Arc::clone(&apple);

        thread::spawn(move || {
            // As Arc was used, threads can be spawned using the value allocated
            // in the Arc variable pointer's location.
            println!("{:?}", apple);
        });
    }
}
```


# Heap Allocation

* Main function is kept in a “main frame” on the Stack
* Every function call is added to the Stack memory as a frame-block
* All static variables including arguments and the return value is saved within the function frame-block on the Stack
* All static values regardless of type are stored directly on the Stack. This applies to global scope as well
* All dynamic types created on the Heap and is referenced from the Stack using smart pointers. This applies to the global scope as well. Here we explicitly made the name dynamic to avoid it going to the Stack as having a fixed-length string value will do that
* The struct with static data is kept on the Stack and any dynamic value in it is kept on the Heap and is referenced via pointers
* Functions called from the current function is pushed on top of the Stack
* When a function returns its frame is removed from the Stack
* Unlike Garbage collected languages, once the main process is complete, the objects on the Heap are destroyed as well, we will see more about this in the following sections

# Why heap?

Shrinking oft-instantiated types can help performance.

For example, if memory usage is high, a heap profiler like DHAT or heaptrack can identify the hot allocation points and the types involved. Shrinking these types can reduce peak memory usage, and possibly improve performance by reducing memory traffic and cache pressure.

Furthermore, Rust types that are larger than 128 bytes are copied with memcpy rather than inline code. If memcpy shows up in non-trivial amounts in profiles, DHAT’s “copy profiling” mode will tell you exactly where the hot memcpy calls are and the types involved. Shrinking these types to 128 bytes or less can make the code faster by avoiding memcpy calls and reducing memory traffic.

  <img src="https://i.imgur.com/8R9esMB.jpg" width="350" title="Heap Allocation">




