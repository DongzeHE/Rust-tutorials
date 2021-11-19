# week 10

Today's topics
1. [closure](https://zhauniarovich.com/post/2020/2020-12-closures-in-rust/)
2. [builder pattern](https://subscription.packtpub.com/book/application_development/9781788623926/1/ch01lvl1sec17/using-the-builder-pattern)
3. [iterator]()

# Closure

* How to define a closure
* How does the compiler know the type of variables used in the closure


## What is a closure?

Rust’s closures are anonymous functions you can save in a variable or pass as arguments to other functions. 

* A closure can be imagined like a function;
* Contrary to a function, it can capture its environment (capturing the environment means that in a closure you can use the variables defined outside the closure body but accessible in its scope).

```rust

fn main() {
    let intro = String::from("name\tage");
    let print_user_age = |name, age| println!("{}\n{}\t{}\n", intro, name, age);

    for (name, age) in [
        (String::from("Alice"), 5),
        (String::from("Bob"), 7),
        (String::from("Mallory"), 20),
    ]
    .iter()
    {
        print_user_age(name, age);
    }
}

```

```rust
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```

Notice that we did not use `intro` as an input variable! Closure capture variables implicitly.

As the types captured by closures can be implicit, how do a closure know which type (ref, mutable ref, or ownership) should be captured?

<img src="https://zhauniarovich.com/post/2020/2020-12-closures-in-rust/closures.svg"
     alt="Markdown Monster icon"
     style="float: left; margin-right: 10px;" />


Non-capturing closures (closures that do not capture variables from its environment) or closures that capture variables only by immutable borrow, implements automatically the Fn trait.
If all variables are captured by mutable and immutable borrows, then the closure implements automatically the FnMut trait.
If at least one variable is captured by move, then the closure implements automatically the FnOnce trait.

## How to use closure?

### capture by immutable borrow

```rust
fn main() {
    let immut_val = String::from("immut");
    let fn_closure = || {
        println!("Len: {}", immut_val.len());
    };

    println!("Value: {}", immut_val); //ok
    fn_closure();                     //ok

    // cannot borrow mutably because it is already borrowed immutably
    // immut_val.push_str("-push");   
    // fn_closure();
}
```

### Capture by mutable borrow

```rust
fn main() {
    let mut mut_val = String::from("mut");
    let mut fnmut_closure = || {
        mut_val.push_str("-new");
    };

    // cannot borrow immutable because the variable is borrowed as mutable
    // println!("{}", mut_val);
    
    // cannot borrow mutably second time
    // mut_val.push_str("another_string");
    
    fnmut_closure();

    // ok because closure is already dropped 
    println!("{}", mut_val); 
}
```

### Capture by move

```rust
fn main() {
    let mov_val = String::from("value");
    let fnonce_closure = || {
        let moved_value = mov_val;
    };
    
    fnonce_closure(); // ok
    
    // cannot print it because it is captured in the closure
    // borrow of moved value
    // println!("{}", mov_val);
    // cannot call closure the second time
    // fnonce_closure();
    
    let immut_val = String::from("hello");
    let mov_closure = move || println!("immut_val");
    
    use std::thread;
        
    let name = String:: from("Alice");
    let print_closure = move || println!("Name: {}", name);
    let handler = thread::spawn(print_closure);
    handler.join().expect("Error happened!");

}
```

# iterator

* Why do we have `into_iter()` AND `iter()` and `iter_mut()`?
* Why cannot call `iter()` on std collections?
* How to write a custom iterator?

## what is an interator

The iterator pattern allows you to perform some task on a sequence of items in turn. An iterator is responsible for the logic of iterating over each item and determining when the sequence has finished. When you use iterators, you don’t have to reimplement that logic yourself.

In Rust, iterators are lazy, meaning they have no effect until you call methods that consume the iterator to use it up. For example, the code in the example below creates an iterator over the items in the vector v1 by calling the iter method defined on `Vec<T>`. This code by itself doesn’t do anything useful.

We use iterator all the time.
```rust
let names = vec!["Pascal", "Elvira", "Dominic", "Christoph"];

for name in names {
    println!("{}", name);
}
```
    
```rust
let mut book_reviews = HashMap::new();

book_reviews.insert(
    "Search Inside Yourself".to_string(),
    "A great book about meditation.".to_string(),
);

book_reviews.insert(
    "Limitless".to_string(),
    "Unleash the potential of your brain!".to_string(),
);

for review in book_reviews {
    println!("{}: {}", review.0, review.1);
}
```

for loops consume any type that implement the `Iterator` trait.

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
    ...
}
```

The `Iterator` trait comes with a `next()` method that returns `Option<Self::Item>`. When it reaches the last itm, `next()` will return nothing. This also means that iterators are stateful because they keep track of where they are in the iteration process. 

Interestingly, `Vec<T>` and `HashMap<K,V>`, and other (and other collection types) do NOT implement the `Iterator` trait. If we try to call next() on them like this:

```rust
let names = vec!["Pascal", "Elvira", "Dominic", "Christoph"];
names.next();
```

```bash
error[E0599]: no method named `next` found for struct `std::vec::Vec<&str>` in the current scope
 --> src/main.rs:9:11
  |
9 |     names.next();
  |           ^^^^ method not found in `std::vec::Vec<&str>`
```

Actually, there’s another trait in place that ensures our loop indeed receives an `Iterator`. That trait is the `IntoIterator` trait.

## Iterables with IntoIterator

When there’s a “natural way” to iterate over some type, it can implement the `IntoIterator` trait. `IntoIterator` comes with an `into_iter()` method that returns an iterator over its value. Here’s what it looks like:

```rust=
trait IntoIterator where Self::IntoIter::Item == Self::Item {
    type Item;
    type IntoIter: Iterator;
    fn into_iter(self) -> Self::IntoIter;
}
```

Any type that implements `IntoIterator` is also called an **Iterable**. So how does this trait play a role in the original scenario we’ve discussed? If we have a for loop that looks like this:

```rust=
let names = vec!["Pascal", "Elvira", "Dominic", "Christoph"];

for name in names {
    println!("{}", name);
}
```

```rust=
let mut iterator = (names).into_iter();
while let Some(name) = iterator.next() {
    println!("{}", name);
}
```

This also works when iterators are passed directly to `for` loops, because any type that implements `Iterator` also implements `IntoIterator`, which then just simply returns the iterator itself:

```rust
let names = vec!["Pascal", "Elvira", "Dominic", "Christoph"];

let iterator = (names).into_iter();

for name in iterator {
    println!("{}", name); 
}
```

You may notice that the `into_iter()` method consumes the variable. So when we don't want the iterator takes the ownership of our data, we usually use `iter()` and `iter_mut()`. Those two methods are implemented for collections.

```rust
let names = vec!["Pascal", "Elvira", "Dominic", "Christoph"];

let mut iterator = (names).iter(); // or iter_mut() respectively

println!("{}", iterator.next().unwrap());
println!("{}", iterator.next().unwrap());
```

As mentioned before, IntoIterator implementations mostly come into play in combination with for loops. One thing to keep in mind here, is that we probably want the flexibility to consume our iterable values to be by value or (mutable) reference depending on our context.

If this doesn’t make a lot of sense to you, you might want to read this article on References in Rust and come back once you’re done.

```rust
for element in &collection { ... }
for element in &mut collection { ... }
for element in collection { ... }
```

```rust
impl<T> IntoIterator for Vec<T>
impl<'a, T> IntoIterator for &'a Vec<T>
impl<'a, T> IntoIterator for &'a mut Vec<T>
```

Depending on how we use `into_iter()` on a `Vec<T>` we’ll get different types of values produced, namely values of `T`, `&T` or `&mut T` respectively (as illustrated in the three for loops above). Keep in mind though, that this only works because `Vec<T>` happens to implement IntoIterator for these three scenarios. 


* Given a shared reference to a collection, `into_iter()` returns an iterator that produces shared references to its items.
* Given a mutable reference to a collection, it returns an iterator that produces mutable references to the items.
* Given a collection as value, it returns an iterator that takes ownership of the collection and returns items by value. For a quick primer on ownership, check out this article.
* `iter()` always returns an iterator that produces shared references to its items.
* `iter_mut()` always returns an iterator that produces mutable references to its items.


### Methods that consume the iterator

Methods can be written to consume the iterator, and these that call next are called consuming adaptors, because calling them uses up the iterator. One example is the sum method, which takes ownership of the iterator and iterates through the items by repeatedly calling next, thus consuming the iterator. As it iterates through, it adds each item to a running total and returns the total when iteration is complete. Listing 13-16 has a test illustrating a use of the sum method.

```rust
    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);
    }
```

We aren’t allowed to use v1_iter after the call to sum because sum takes ownership of the iterator we call it on.

### Methods that Produce Other Iterators

Another kind of method is known as an iterator adaptor, which can allow you to change iterators into different kinds of iterators. You can chain multiple calls to iterator adaptors to perform complex actions in a readable way. But because all iterators are lazy, you have to call one of the consuming adaptor methods to get results from calls to iterator adaptors, as shown below. 

```rust
let v1: Vec<i32> = vec![1, 2, 3];
v1.iter().map(|x| x + 1);
```

However, the following code doesn't do anything since the closure we specified never gets called. The reason for this is that iterator adaptors are lazy, and will only consume the iterator when needed. 

We can finish this example as shown below.

```rust
let v1: Vec<i32> = vec![1, 2, 3];

let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

assert_eq!(v2, vec![2, 3, 4]);
```

### Filter

We can now demonstrate a common use of closures that capture their environment by using the filter iterator adaptor. The filter method on an iterator takes a closure that takes each item from the iterator and returns a Boolean. If the closure returns true, the value will be included in the iterator produced by filter. If the closure returns false, the value won’t be included in the resulting iterator.

In the following example, we use filter with a closure that captures the shoe_size variable from its environment to iterate over a collection of Shoe struct instances. It will return only shoes that are the specified size.

```rust
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}
```

## Custom iterator

```rust
/// An iterator that returns numbers from 1 to 10.
struct OneToTen {
    current: u32,
}

impl OneToTen {
    /// Constructor for OneToTen.
    fn new() -> Self {
        Self {
            current: 0,
        }
    }
}

impl Iterator for OneToTen {
    type Item = u32;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.current < 10 {
            self.current += 1;
            return Some(self.current);
        }
        
        None
    }
}
```

```rust=
struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    // We can refer to this type using Self::Item
    type Item = u32;
    
    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr + self.next;

        self.curr = self.next;
        self.next = new_next;

        Some(self.curr)
    }
}

// Returns a Fibonacci sequence generator
fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 0, next: 1 }
}

fn main() {
    // `0..3` is an `Iterator` that generates: 0, 1, and 2.
    let mut sequence = 0..3;

    println!("Four consecutive `next` calls on 0..3");
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());

    // The `take(n)` method reduces an `Iterator` to its first `n` terms.
    println!("The first four terms of the Fibonacci sequence are: ");
    for i in fibonacci().take(4) {
        println!("> {}", i);
    }

    // The `skip(n)` method shortens an `Iterator` by dropping its first `n` terms.
    println!("The next four terms of the Fibonacci sequence are: ");
    for i in fibonacci().skip(4).take(4) {
        println!("> {}", i);
    }

    let array = [1u32, 3, 3, 7];
```

# builder pattern

Why do we need the builder pattern?


## What is the builder pattern?

> The builder pattern is a design pattern designed to provide a flexible solution to various object creation problems in object-oriented programming. The intent of the Builder design pattern is to separate the construction of a complex object from its representation. It is one of the Gang of Four design patterns. --Wikipedia

# Why do we need the builder pattern?

Some data structures are complicated to construct, due to their construction needing:
* a large number of inputs
* compound data (e.g. slices)
* optional configuration data
* choice between several flavors

In these cases you may want to create a *builder* for the type. You can do this by introducing a separate data type `TBuilder` for *incrementally* configuring a `T` value (though when possible, choose a better name.) The builder constructor should take as parameters only the data required to to make a `T`, and the builder should offer a suite of convenient methods for configuration, including setting up compound inputs (like slices) incrementally. Finally, the builder should provide one or more "terminal" methods for actually building a T.

The advantages of using the builder pattern include
* Separates methods for building from other methods.
* Prevents proliferation of constructors
* Can be used for one-liner initialisation as well as more complex construction.

## Command and Child

In rust std, the struct `std::process::Command` is a builder of struct `std::process::Child`, which represents the child processes in rust. 

The `Command` stuct is defined as 

```rust
// NOTE: the actual Command API does not use owned Strings;
// this is a simplified version.

pub struct Command {
    program: String,
    args: Vec<String>,
    cwd: Option<String>,
    // etc
}

impl Command {
    pub fn new(program: String) -> Command {
        Command {
            program: program,
            args: Vec::new(),
            cwd: None,
        }
    }

    /// Add an argument to pass to the program.
    pub fn arg<'a>(&'a mut self, arg: String) -> &'a mut Command {
        self.args.push(arg);
        self
    }

    /// Add multiple arguments to pass to the program.
    pub fn args<'a>(&'a mut self, args: &[String])
                    -> &'a mut Command {
        self.args.push_all(args);
        self
    }

    /// Set the working directory for the child process.
    pub fn cwd<'a>(&'a mut self, dir: String) -> &'a mut Command {
        self.cwd = Some(dir);
        self
    }

    /// Executes the command as a child process, which is returned.
    pub fn spawn(&self) -> IoResult<Process> {
        ...
    }
}
```

As you can see, the only purpose of building a `Command` struct is to use it to construct the `Child` stuct, i.e., a child process. This is where the name the "builder" pattern comes from.

```rust
use std::process::Command;

fn main() {
    let output = Command::new("sh")
        .arg("-c")
        .arg("echo hello")
        .output()
        .expect("fail to execute process.");
    println!("{}",String::from_utf8(output.stdout).unwrap());
}
```

### Consuming builders

Sometimes builders must transfer ownership when constructing the final type `T`, meaning that the terminal methods must take `self` rather than `&self`:

```rust
// A simplified excerpt from std::thread::Builder

impl ThreadBuilder {
    /// Name the thread-to-be. Currently the name is used for identification
    /// only in failure messages.
    pub fn named(mut self, name: String) -> ThreadBuilder {
        self.name = Some(name);
        self
    }

    /// Redirect thread-local stdout.
    pub fn stdout(mut self, stdout: Box<Writer + Send>) -> ThreadBuilder {
        self.stdout = Some(stdout);
        //   ^~~~~~ this is owned and cannot be cloned/re-used
        self
    }

    /// Creates and executes a new child thread.
    pub fn spawn(self, f: proc():Send) {
        // consume self
        ...
    }
}

```

One-liners work as before, because ownership is threaded through each of the builder methods until being consumed by spawn. Complex configuration, however, is more verbose: it requires re-assigning the builder at each step.

```rust
// One-liners
ThreadBuilder::new().named("my_thread").spawn(proc() { ... });

// Complex configuration
let mut thread = ThreadBuilder::new();
thread = thread.named("my_thread_2"); // must re-assign to retain ownership

if reroute {
    thread = thread.stdout(mywriter);
}

thread.spawn(proc() { ... });
```


## A burger builder

can be found at [here](https://subscription.packtpub.com/book/application_development/9781788623926/1/ch01lvl1sec17/using-the-builder-pattern).


