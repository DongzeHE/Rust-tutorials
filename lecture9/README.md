# Week 9

resource: [rust-gentle-intro](https://stevedonovan.github.io/rust-gentle-intro/object-orientation.html), [TRPL](https://doc.rust-lang.org/book/ch17-00-oop.html) and [RBE](https://doc.rust-lang.org/rust-by-example/trait.html).

This week we will focus on objected-oriented programming features in rust.


Rust is sort of a Object-Oriented Language.
Arguably, OOP languages share certain common characteristics, namely objects, encapsulation, and inheritance. Let’s look at what each of those characteristics means and whether Rust supports it.

# 1. Objects Contain Data and Behavior

> Object-oriented programs are made up of objects. An object packages both data and the procedures that operate on that data. The procedures are typically called methods or operations.[name=The Gang of Four book]

Using this definition, Rust is object oriented: structs and enums have data, and `impl` blocks provide methods on structs and enums. 

```rust
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn new(width: u32, height: u32) -> Rectangle {
        Rectangle {
            width, 
            height,
        }
    }
}
```

# 2. Encapsulation that Hides Implementation Details

The option to use `pub` or not for different parts of code enables encapsulation of implementation details. By default, everything is private. If we want to access methods or functions in public API, we need to specify the `pub` keyword before our data type.

```rust
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            },
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}
```

The public methods `add`, `remove`, and `average` are the only way to modify an instance of `AveragedCollection`. When an item is added to list using the `add` method or removed using the `remove` method, the implementations of each call the private `update_average` method that takes care of updating the average field as well.

We leave the `list` and `average` fields private so that there’s no way for external code to add or remove items to the list field directly, otherwise the `average` field might become out of sync when the list changes. The `average` method returns the value in the `average` field, allowing external code to read the average but not modify it.

Because we’ve encapsulated the implementation details of `AveragedCollection`, we can easily change aspects like the data structure in the future. For instance, we could use a `HashSet` instead of a `Vec` for the list field. As long as the signatures of the `add`, `remove`, and `average` public methods stay the same, code using `AveragedCollection` wouldn’t need to change. If we made `list` public instead, this wouldn’t necessarily be the case: `HashSet` and `Vec` have different methods for adding and removing items, so the external code would likely have to change if it was modifying list directly.

If encapsulation is a required aspect for a language to be considered object-oriented, then Rust meets that requirement. The option to use `pub` or not for different parts of code enables encapsulation of implementation details.

# 3. Inheritance as a Type System and as Code Sharing

Inheritance is a mechanism whereby an object can inherit from another object’s definition, thus gaining the parent object’s data and behavior without you having to define them again.

In Rust, there is no way to define a struct that inherits the parent struct’s fields and method implementations. However, rust provide some solutions, depending on your reason for reaching for inheritance in the first place.

## Why doesn't Rust use inheritance? 
Inheritance is often **at risk of sharing more code than necessary**. Subclasses *shouldn’t* always share all characteristics of their parent class but will do so with inheritance. This can make a program’s design less flexible. It also introduces the possibility of calling methods on subclasses that don’t make sense or that cause errors because the methods don’t apply to the subclass. In addition, some languages will only allow a subclass to inherit from one class, further restricting the flexibility of a program’s design.

## What are the substitutions?

You choose inheritance for two main reasons. 
- One is for reuse of code: you can implement particular behavior for one type, and inheritance enables you to reuse that implementation for a different type. 
- The other reason to use inheritance *relates to the type system: to enable a child type to be used in the **same places** as the parent type*. This is also called *polymorphism*, which means that you can substitute multiple objects for each other at runtime if they share certain characteristics. the trait object `Box<dyn Trait>` is the secret. When talking about generics, we said the compiler will figure out what type a generics is at compile time.

> **Polymorphism**
> To many people, polymorphism is synonymous with inheritance. But it’s actually a more general concept that refers to code that can work with data of multiple types. For inheritance, those types are generally subclasses.
>Rust instead uses generics to abstract over different possible types and trait bounds to impose constraints on what those types must provide. This is sometimes called bounded parametric polymorphism.

## Reuse of code

There are many ways in rust that can help you write less code. Generics is one. Default implementation of your trait is another one.

### Generics

#### Generics in struct/enum

struct/enum can take generic type. Implementation of struct/enum can also take generic type.

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

enum Option<T> {
    Some(T),
    None,
}
```

#### Generics in traits

Instead of implementing functions for every concrete type, generics provides Rust a template for implementing functions for concrete types. For example, `Option<T>`, `HashMap<K,V>`. 

```rust
// Non-copyable types.
struct Empty;
struct Null;

// A trait generic over `T`.
trait DoubleDrop<T> {
    // Define a method on the caller type which takes an
    // additional single parameter `T` and does nothing with it.
    fn double_drop(self, _: T);
}

// Implement `DoubleDrop<T>` for any generic parameter `T` and
// caller `U`.
impl<T, U> DoubleDrop<T> for U {
    // This method takes ownership of both passed arguments,
    // deallocating both.
    fn double_drop(self, _: T) {}
}

fn main() {
    let empty = Empty;
    let null  = Null;

    // Deallocate `empty` and `null`.
    empty.double_drop(null);

    //empty; ERROR
    //null; ERROR
}
```

#### Performance of Code Using Generics
Rust doesn’t run any slower if using generic types. Rust accomplishes this by performing monomorphization of the code that is using generics at compile time. Monomorphization is the process of turning generic code into specific code by filling in the concrete types that are used when compiled.

### Traits

Trait can help make our implementation more concise.

#### Trait bounds
By specifying the trait bounds of function arguments, we are free from implementing the function for every concrete type.

```rust
use std::fmt::{Debug, Display};

fn compare_prints<T: Debug + Display>(t: &T) {
    println!("Debug: `{:?}`", t);
    println!("Display: `{}`", t);
}

fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
    println!("t: `{:?}`", t);
    println!("u: `{:?}`", u);
}

fn main() {
    let string = "words";
    let array = [1, 2, 3];
    let vec = vec![1, 2, 3];

    compare_prints(&string);
    // compare_prints(&array); // ERROR

    compare_types(&array, &vec);
}

```

#### Default implementation of trait

Instead of implementing the functions or methods required by a trait in each type that implements the trait, providing a default implementation can save some space. 
```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
```

### Polymorphism

The second reason to use inheritance relates to the type system: to enable a child type to be used in the same places as the parent type. This is also called polymorphism, which means that multiple objects can be substituted for each other at runtime if they share certain characteristics. 

As we said, inheritance has recently fallen out of favor as a programming design solution because it is often at risk of sharing more code than needs be. For rust, its unsafe! Instead, the solution in rust is trait. It is clear that traits the bad things of inheritance are not in trait:
1. A trait defines only the *minimum* behaviors.
2. Default implementation of traits *don't involve any* concreate fields.
3. All functions and methods in a trait *must* be implemented, either default or specific.

#### The orphan rules in rust

Recall the orphan rule: if you want to implement a trait for your data type, at least one of them must be local. This means you cannot implement `Deref` trait for `Vec`, because both of them are remote, i.e., from other crates. One trick is that you can wrap a vector into a custom struct, then you can implemtn the `Deref` trait for the struct if you really want to equip a vector with this trait.

#### Defining a trait object for common behavior
A trait object `&dyn Trait` points to both an instance of a **pointer type** implementing our specified trait as well as a table used to look up trait methods on that type at runtime. 

The Rust compiler restricts that all the values in a vector **must** have the same type. Even if we define a generic type, the generic type can be substituted with **one** contrete type at a time.

```rust
fn main() {
    // Vec<T>
    let v1 = vec![1u32, 1u32]; 
    let v2 = vec![1u64, 1u64]; 
    let v3 = vec![1u32, 1u64]; // ERROR
}
```

A **Trait object** tells rust to pass all types that implement the trait.

```rust
trait Show {
    fn show(&self) -> String;
}

impl Show for i32 {
    fn show(&self) -> String {
        format!("four-byte signed {}", self)
    }
}

impl Show for f64 {
    fn show(&self) -> String {
        format!("eight-byte float {}", self)
    }
}

fn main() {
    let answer = 42;
    let maybe_pi = 3.14;
    let v: Vec<&dyn Show> = vec![&answer,&maybe_pi];
    for d in v.iter() {
        println!("show {}",d.show());
    }
}
// show four-byte signed 42
// show eight-byte float 3.14
```

If you want your type to own the data, you may want to wrap your data in a smart pointer, such as `Box<dyn trait>`

```rust
struct Sheep {}
struct Cow {}

trait Animal {
    // Instance method signature
    fn noise(&self) -> &'static str;
}

// Implement the `Animal` trait for `Sheep`.
impl Animal for Sheep {
    fn noise(&self) -> &'static str {
        "baaaaah!"
    }
}

// Implement the `Animal` trait for `Cow`.
impl Animal for Cow {
    fn noise(&self) -> &'static str {
        "moooooo!"
    }
}

// Returns some struct that implements Animal, but we don't know which one at compile time.
fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}

fn main() {
    let random_number = 0.234;
    let animal = random_animal(random_number);
    println!("You've randomly chosen an animal, and it says {}", animal.noise());
}
```

# Implementing an Object-oriented design pattern

One OOP design pattern is called *state pattern*. It means that some state object has some internal state, and the object's behavior changes based on the internal state.  

We’ll implement a blog post workflow in an incremental way. The blog’s final functionality will look like this:

1. A blog post starts as an empty draft.
1. When the draft is done, a review of the post is requested.
1. When the post is approved, it gets published.
1. Only published blog posts return content to print, so unapproved posts can’t accidentally be published.

So, our API will be something like 

Filename: src/main.rs
```rust
use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
```

Obviously, the `post` type contains data and behavior, hides its implementation details and the objects in different states share some behaviors (all of them have `content()` method).

As a smart Rust programmer, we decide to organize the implementation like this.

```rust
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft)),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}


struct Draft;

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview)
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview;

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published)
    }
}

struct Published;

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
```


