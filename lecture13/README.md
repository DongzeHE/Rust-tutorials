# lecture 13
 
# Ownership

Ownership is the Rust way to manage memory. The main idea is that each memory chunk is owned by one variable. Although we saw that some smart pointers could fool the compiler and be owned by multiple variables, they still ensure memory safety in their way. 

Ownership defines the behavior of the variables who own their data.

```rust
let n = 5; // stack data, copy trait
let v = vec![1,2,3]; // heap data, clone trait
{
let n1 = n;
let v1 = v; // ownership transferred
println!("{:?}",n); // 
println!("{:?}",v); // ERROR. access after move
}
println!("{:?}",n1); // ERROR. value dropped 
```

## Ownership rules
* Each value in Rust has a variable that's called its owner.
* There can only be one owner at a time.
* When the owner goes out of scope, the value will be dropped.

## Ownership in a function call
Each function has its scope. When the function call is over, the scope is over. A function is a chunk of code that processes its input and dumps some data if asked. So, when you call a function, the compiler will create a new scope `{a scope}` and evaluate the function in the scope.

```rust
fn add(a: i32, b: i32) -> i32 {
    let var = 5;
    a + b
} // var is dropped

fn a_ref() -> &i32 {
    let var = 5;
    &var
} // var is dropped, &var is invalid

// So, the following two examples are same.
fn main() {
    let a = 5;
    let b = 6;
    let c = {
        let var = 5; 
        a + b
    }; // var is dropped
    
    let d = add(a, b);
    
    let r = {
        let var = 5;
        &var
    } // ERROR
}
```

So, if you define a variable in a function, it will be dropped when the scope of the function is over unless you use this variable as the return.  

# lifetime
Ownership defines the behavior of data owners. Lifetime defines the behavior of data borrowers. We talk about the lifetime when we have a reference or smart pointer.

```rust
let n1 = 1;
let mut n2 = 5;

let r1 = &mut n1; // ERROR, n1 is immutable
{
    let r2 = &n2; // r2 is immutable
    let r3 = &mut n2; // ERROR, mut OR immut
}

{
    let r4 = &mut n2; // r4 is mutable
}
```

## lifetime rules
* At any given time, you can have either one mutable reference or any number of immutable references.
* References must always be valid.

## Explicit lifetime

Most times, the compiler will figure out the lifetime of the pointers for you, but you need to specify them by yourself. 

Lifetime elision rules

* The first rule is that each parameter that is a reference gets its own lifetime parameter. In other words, a function with one parameter gets one lifetime parameter: `fn foo<'a>(x: &'a i32);` a function with two parameters gets two separate lifetime parameters: `fn foo<'a, 'b>(x: &'a i32, y: &'b i32);` and so on.
* The second rule is if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters: `fn foo< 'a>(x: & 'a i32) -> & 'a i32`.
* The third rule is if there are multiple input lifetime parameters, but one of them is `&self` or `&mut self` because this is a method, the lifetime of `self` is assigned to all output lifetime parameters. This third rule makes methods much nicer to read and write because fewer symbols are necessary.

One example we have seen that needs explicit lifetime annotation is

```rust
fn longest<'b, 'a: 'b>(x: &'a str, y: &'b str) -> &'b str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

# Smart pointer and trait object
Smart pointers are pointers that point at some heap memory. Smart pointers usually store some metadata. 
We introduced three kinds of smart points, `Box`, `Rc`, which stands for Reference counting, and `RefCell`. We also have seen trait objects defined by the `dyn` keyword.

## Box

Box is the most straightforward smart pointer. It just contains a memory address of some heap data. It has no metadata.

```rust
use std::{cell::RefCell, mem, rc::Rc;

fn main() {
    let n = 5usize; // stack data
    let s = "I am in a smart pointer".to_string();
    let b = Box::new(s.clone()); // heap data
    println!("usize size = {}", mem::size_of_val(&n)); // 8
    println!("Box size = {}", mem::size_of_val(&b)); // 8
    println!("inner data size = {}", mem::size_of_val(&(*b))); // 24
    println!("String size = {}", mem::size_of_val(&s)); // 24
}
```

A `Box` is just a pointer that points at a piece of heap data. The size of a `Box` is always equal to `usize`. You can use a `Box` as a reference.
![](https://i.imgur.com/Vu4LCR1.png =100x)

## trait object

The keyword `dyn Trait` defines a trait object, where `Trait` is a placeholder of a trait. A trait object is an opaque value of another type that implements a set of traits. The set of traits is made up of an object safe base trait plus any number of auto traits. 

Usually, we want to put a trait object into a `Box` and use the pointer instead of moving the ownership around. So `Box<dyn Trait>` is the format we usually see. Wherever we use a trait object, Rust's type system will ensure at compile time that any value used in that context will implement the trait object's trait. Using trait objects can cheat the compiler if there is any size or type restriction. 

```rust
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}
```

`Box` has fixed size (`usize`) and type(`Box`), so the compiler will regard this vector as a plain vector. However, we can put anything in the `Box`, a struct, a vector, a string, or an iron man, as long as we implement the desired `Trait` for it. 

### Rc

`Rc` stands for reference counting. It maintains two counts, `strong_count` and `weak_count`. If you do `let rc1 = Rc::clone(&rc)`, you will get another pointer that points at the same `Rc` that the original `rc` points at, and the `strong_count` of that `Rc` increases 1. 

Using `Rc` means we can assign the ownership of one memory chunk to multiple variables via creating pointers. So the owners owns the pointers instead of the memory chunk. `Rc` will make sure that its inner data is accessed by at most one owner at any time. 

```rust
use std::rc::Rc;

fn main() {
    let a = Rc::new("I am a Rc".to_string());
    let b = Rc::clone(&a);
    println!("{}",a);
    println!("{}",b);
}
```
![](https://i.imgur.com/6uhGzmN.png =200x)

You can use the owners of a `Rc` like references. 


### RefCell

`RefCell` is actually the reference version of `Cell`, which provides interior mutability. We can make a `RefCell` as mutable without `mut` keyword. 

Interior mutability is when you have an immutable reference (i.e., `&T`) but you can mutate the data structure. When you clone an `Rc` (`Rc::clone(&rc)`) or lock a `Mutex` (both `Mutex::lock(&mutex)` and `Mutex::try_lock(&mutex)` work in immutable instances).

```rust
struct NaiveRc<T> {
    reference_count: usize,
    inner_value: T,
}

impl Clone for NaiveRc<T> {
    fn clone(&self) -> Self {
        self.reference_count += 1; // we are editing the value with imutable reference
        // ...
    }
}
```

We need to update the reference count of the `Rc` when cloning, but the `Clone()` method takes an immutable reference as the input. Here we want to use interior mutability.

```rust
use std::cell::RefCell;

struct NaiveRc<T> {
    inner_value: T,
    references: RefCell<usize>,
}

impl<T> NaiveRc<T> {
    fn new(inner: T) -> Self {
        NaiveRc {
            inner_value: inner,
            references: RefCell::new(1),
        }
    }

    fn references(&self) -> usize {
        *RefCell::borrow(&self.references)
    }
}

impl<T: Clone> Clone for NaiveRc<T> {
    fn clone(&self) -> Self {
        *self.references.borrow_mut() += 1;
        NaiveRc {
            inner_value: self.inner_value.clone(),
            references: self.references.clone(),
        }
    }
}

fn main() {
    let wrapped = NaiveRc::new("Hello!");
    println!("references before cloning: {:?}", wrapped.references());
    let wrapped_clone = wrapped.clone();
    println!("references after cloning: {:?}", wrapped.references());
    println!("clone references: {:?}", wrapped_clone.references());
}
```

## `Rc<RefCell<Box<dyn Trait>>>`

```rust
use std::{cell::RefCell, rc::Rc};

trait TestTrait {
    fn say_something(&self);
}

struct TestStruct1;
impl TestTrait for TestStruct1 {
    fn say_something(&self) {
        println!("I am a TestStruct1");
    }
}
struct TestStruct2;
impl TestTrait for TestStruct2 { 
    fn say_something(&self) {
        println!("I am a TestStruct2");
    }
}
```

Now let's explain this monster. 
- `Box<dyn Trait>` is a trait object. We use this to give our `RefCell` the ability to take more than one type as the inner data, and make the size of the inner data as fixed (`usize`).
    ```rust
    fn main() {
        let ts1 = TestStruct1;
        let ts2 = TestStruct2;
        let mut v: Box<dyn test_trait> = Vec::new();
        v.push(Box::new(ts1));
        v.push(Box::new(ts2));
    }
    ```


- `RefCell<Box<dyn Trait>>` give our type the interior mutability. 
    ```rust
    fn main() {
        let ts1 = TestStruct1;
        let ts2 = TestStruct2;
        let v: Vec<Rc<RefCell<Box<dyn TestTrait>>>> = vec![Rc::new(RefCell::new(Box::new(ts1)))];
        RefCell::borrow(&v[0]).say_something();
        *RefCell::borrow_mut(&v[0]) = Box::new(ts2);
        RefCell::borrow(&v[0]).say_something();
    }
    ```
- `Rc<RefCell<Box<dyn Trait>>>` allows our type to have multiple owners. 
    ```rust
    fn main() {
        let ts1 = TestStruct1;
        let ts2 = TestStruct2;
        let v: Vec<Rc<RefCell<Box<dyn TestTrait>>>> = vec![Rc::new(RefCell::new(Box::new(ts1)))];    
        let rc = Rc::clone(&v[0]);
        RefCell::borrow(&rc).say_something();
    }
    ```

In conclusion, our type can have multiple owners. Each owner can modify the inner value even when the type is immutable (no `mut` keyword). Moreover, the inner data of our type can be any type that implements the `TestTrait` trait.  

# ownership for function call
how ownership rules work when returning something from a function call (returning a struct directly vs a mutable reference vs a box vs a RC vs a refcell, etc.)

When your function returns something, it moves the ownership of the returned value to the LHS of your function call. Then, you can use ownership rules for the return type.

## Ownership rules
* Each value in Rust has a variable that's called its owner.
* There can only be one owner at a time.
* When the owner goes out of scope, the value will be dropped.


- Could we go through examples of designing structs to accomplish certain tasks and requirements?
[Rust Design Pattern](https://rust-unofficial.github.io/patterns/)

- Clippy
[clippy lints](https://rust-lang.github.io/rust-clippy/master/)

- [Rust Container Cheat Sheet](https://docs.google.com/presentation/d/1q-c7UAyrUlM-eZyTo1pd8SZ0qwA_wYxmPZVOQkoDmH4/edit#slide=id.p)


