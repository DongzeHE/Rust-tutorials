# week 11

1. Fearless concurrency
2. panthom type

Resource: [TRPL](https://doc.rust-lang.org/book/ch16-00-concurrency.html) and [RBE](https://doc.rust-lang.org/rust-by-example/generics/phantom.html)

# Fearless concurrency

*Concurrent programming*, where different parts of a program execute independently, and *parallel programming*, where different parts of a program execute at the same time, are becoming increasingly important as more computers take advantage of their *multiple processors*.

In rust, the ownership and type systems are a powerful set of tools to help manage memory safety *and* concurrency problems.

## Using threads to run code simultaneously

In most current operating systems, an executed program’s code is run in a *process*, and the operating system manages multiple processes at once. Within your program, you can also have independent parts that run simultaneously. The features that run these independent parts are called *threads*.

Multi-threaded task can lead to the follwoing problems:

* Race conditions, where threads are accessing data or resources in an inconsistent order
* Deadlocks, where two threads are waiting for each other to finish using a resource the other thread has, preventing both threads from continuing
* Bugs that happen only in certain situations and are hard to reproduce and fix reliably

Rust std provides 1:1 threading, meaning one operating system thread per one language thread. There are crates that provide M:N threading in rust, such as [futures](https://docs.rs/futures/0.3.5/futures/index.html),but we will focus on the theading methods in Rust std.

### Creating a New Thread with spawn

Creating threads in rust is easy, we call the `thread::spawn()` function and pass it a closure containing the code we want to run in the new thread. 

```rust
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    
    // handle.join().unwrap(); 
}
```

The threads will probably take turns, but that isn’t guaranteed: it depends on how your operating system schedules the threads. In this run, the main thread printed first, even though the print statement from the spawned thread appears first in the code. And even though we told the spawned thread to print until i is 9, it only got to 5 before the main thread shut down.

We can fix the problem of the spawned thread not getting to run, or not getting to run completely, by saving the return value of `thread::spawn` in a variable. The return type of `thread::spawn` is `JoinHandle`. A `JoinHandle` is an owned value that, when we call the join method on it, will wait for its thread to finish.

Notice that the position where you put the `join()` at can affect whether or not your threads run at the same time.

### Using move Closures with Threads

we can use the `move` keyword before the parameter list of a closure to force the closure to take ownership of the values it uses in the environment. 

To use data from the main thread in the spawned thread, the spawned thread’s closure must capture the values it needs. 

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(/*move*/ || {
        println!("Here's a vector: {:?}", v);
    });
    
    // drop(v); // oh no!
    handle.join().unwrap();

}
```

Rust infers how to capture `v`, and because `println!` only needs a reference to `v`, the closure tries to borrow `v`. However, there’s a problem: Rust can’t tell how long the spawned thread will run, so it doesn’t know if the reference to `v` will always be valid . So, we can use `move` to solve this problem. However, this also means we cannot use `v` anymore after calling `spawn`!

## Using Message Passing to Transfer Data Between Threads

 Here’s the idea in a slogan from the [Go language documentation](https://golang.org/doc/effective_go.html#concurrency): “Do not communicate by sharing memory; instead, share memory by communicating.”

One major tool Rust has for accomplishing message-sending concurrency is the *channel*, a programming concept that Rust’s standard library provides an implementation of. You can imagine a channel in programming as being like a channel of water, such as a stream or a river. If you put something like a rubber duck or boat into a stream, it will travel downstream to the end of the waterway.

We create a new channel using the `mpsc::channel` function; `mpsc` stands for *multiple producer, single consumer*. 

In short, the way Rust’s standard library implements channels means a channel can have multiple sending ends that produce values but only one receiving end that consumes those values. Imagine multiple streams flowing together into one big river: everything sent down any of the streams will end up in one river at the end. We’ll start with a single producer for now, but we’ll add multiple producers when we get this example working.

The `mpsc::channel` function returns a tuple, the first element of which is the sending end and the second element is the receiving end. The abbreviations tx and rx are traditionally used in many fields for transmitter and receiver respectively, so we name our variables as such to indicate each end. We’re using a let statement with a pattern that destructures the tuples. 

Let’s move the transmitting end into a spawned thread and have it send one string so the spawned thread is communicating with the main thread. This is like putting a rubber duck in the river upstream or sending a chat message from one thread to another.

```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // (transmitter, receiver)
    let (tx, rx) = mpsc::channel();
    
    // clone the transmitter so that each child thread gets one
    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("one rubber duck in river 1"),
            String::from("two rubber ducks in river 1"),
            String::from("three rubber ducks in river 1"),
            String::from("four rubber ducks in river 1"),
        ];
        // putting a rubber duck in the river upstream
        for val in vals {
            // send() returns a Result
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
        // send takes the ownership of val
        // println!("{}",val); ERROR!
    });
    
        thread::spawn(move || {
        let vals = vec![
            String::from("one rubber duck in river 2"),
            String::from("two rubber ducks in river 2"),
            String::from("three rubber ducks in river 2"),
            String::from("four rubber ducks in river 2"),
        ];
        // putting a rubber duck in the river upstream
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
        // println!("{}",val); ERROR!
    });
    // rx blocks the main thread!
    // it implements the Iterator trait
    for received in rx {
        println!("Got: {}", received);
    }
}
```

Notice that the spawned thread needs to own the transmitting end of the channel to be able to send messages through the channel. The `send` tries to the value to the receiver end and returns a `Result<T, E>` type.

## Shared-State Concurrency

In a way, channels in any programming language are similar to single ownership, because once you transfer a value down a channel, you should *no longer use* that value. Shared memory concurrency is like *multiple ownership*: multiple threads can access the same memory location at the same time. 

What does this remind you? `Rc` right? The value stored in a `Rc` can have multiple ownership.

### Using Mutexes to Allow Access to Data from One Thread at a Time

The secret is `Mutex` smart pointer. Mutex is an abbreviation for *mutual exclusion*, as in, a mutex allows only one thread to access some data at any given time. Like `RefCell`, `Mutex` provides interior mutability.

To access the data in a mutex, a thread *must first signal that it wants access* by asking to acquire the mutex’s lock. The lock is a data structure that is part of the mutex that keeps track of who currently has exclusive access to the data. Therefore, the mutex is described as guarding the data it holds via the locking system. When using mutexes, you need to remember that 

* You must attempt to acquire the lock before using the data.
* When you’re done with the data that the mutex guards, you must unlock the data so other threads can acquire the lock.

```rust
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    } \\ unlock

    println!("m = {:?}", m);
}
```
As with many types, we create a `Mutex<T>` using the associated function new. To access the data inside the mutex, we use the lock method to acquire the lock. This call will block the current thread so it can’t do any work until it’s our turn to have the lock.

The call to `lock` would fail if another thread holding the lock panicked. In that case, no one would ever be able to get the lock, so we’ve chosen to `unwrap` and have this thread panic if we’re in that situation.

After we’ve acquired the lock, we can treat the return value, named num in this case, as a mutable reference to the data inside. The type system ensures that we acquire a lock before using the value in m: `Mutex<i32>` is not an `i32`, so we must acquire the lock to be able to use the `i32` value. We can’t forget; the type system won’t let us access the inner `i32` otherwise.

As you might suspect, `Mutex<T>` is a *smart pointer*. More accurately, the call to lock returns a smart pointer called MutexGuard, wrapped in a `LockResult` that we handled with the call to unwrap. The `MutexGuard` smart pointer implements `Deref` to point at our inner data; the smart pointer also has a `Drop` implementation that *releases the lock automatically* when a `MutexGuard` goes out of scope. As a result, we don’t risk forgetting to release the lock and blocking the mutex from being used by other threads because the lock release happens automatically.

After dropping the lock, we can print the mutex value and see that we were able to change the inner `i32` to 6.

### Sharing a `Mutex<T>` Between Multiple Threads

 As `move` will take the ownership of `Mutex` if we use it directly in `spawn`. So if we want to share `Mutex<T>` between multiple threads, we need to give the ownership of the `Mutex` value to every thread. To do this, we use the atomic reference counting smart pointer `Arc<T>`, where atomic means it’s an atomically reference counted type that can be sent across threads.

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

### Extensible Concurrency with the Sync and Send Traits

As we said before, the behavior the the types in Rust are defined by traits. So, what are the traits that make `Arc<T>` so powerful? The answer is `Sync` and `Send`.

- The `Send` marker trait indicates that ownership of values of the type implementing Send can be transferred between threads. Almost every Rust type is Send, but there are some exceptions, including `Rc<T>`
- The `Sync` marker trait indicates that it is safe for the type implementing Sync to be referenced from multiple threads. Primitive types are Sync, and types composed entirely of types that are Sync are also Sync. `Rc<T>` is also not `Sync`.

Implementing `Sync` and `Send` needs unsafe rust. If you are interesting in doing it, [The Rustonomicon](https://doc.rust-lang.org/nomicon/index.html) is a good resource.

# Marker types
Marker types don’t have any methods to implement. They’re just useful for enforcing invariants related to concurrency. `Send` and `Sync` are marker traits.

## Panthom types

In rust, if you define a generic type, for exmaple when defining `Slice<'a, T:'a>`, you have to use `'a` and `T` somewhere in your type, such as specifying the type of the fields. 

As a smart coder, if you know that the underlying data is only valid for the lifetime `'a`, so Slice should not outlive `'a`, but there is  no uses of the lifetime `'a` in your struct, you can tell the compiler to act as if the `Slice` struct contained a reference `&'a T`:

```rust
use std::marker::PhantomData;

struct Slice<'a, T: 'a> {
    start: *const T,
    end: *const T,
    phantom: PhantomData<&'a T>,
}

fn borrow_vec<T>(vec: &Vec<T>) -> Slice<'_, T> {
    let ptr = vec.as_ptr();
    Slice {
        start: ptr,
        end: unsafe { ptr.add(vec.len()) },
        phantom: PhantomData,
    }
}
```

If you define a lifetime parameter `'a` in your type, you need to use it 

### Unused type parameters
A phantom type parameter is one that doesn't show up at runtime, but is checked statically (and only) at compile time.

Data types can use extra generic type parameters to act as markers or to perform type checking at compile time. These extra parameters hold no storage values, and have no runtime behavior.

```rust
use std::marker::PhantomData;

// A phantom tuple struct which is generic over `A` with hidden parameter `B`.
#[derive(PartialEq)] // Allow equality test for this type.
struct PhantomTuple<A, B>(A,PhantomData<B>);

// A phantom type struct which is generic over `A` with hidden parameter `B`.
#[derive(PartialEq)] // Allow equality test for this type.
struct PhantomStruct<A, B> { first: A, phantom: PhantomData<B> }

// Note: Storage is allocated for generic type `A`, but not for `B`.
//       Therefore, `B` cannot be used in computations.

fn main() {
    // Here, `f32` and `f64` are the hidden parameters.
    // PhantomTuple type specified as `<char, f32>`.
    let _tuple1: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);
    // PhantomTuple type specified as `<char, f64>`.
    let _tuple2: PhantomTuple<char, f64> = PhantomTuple('Q', PhantomData);

    // Type specified as `<char, f32>`.
    let _struct1: PhantomStruct<char, f32> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };
    // Type specified as `<char, f64>`.
    let _struct2: PhantomStruct<char, f64> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };
    
    // Compile-time Error! Type mismatch so these cannot be compared:
    //println!("_tuple1 == _tuple2 yields: {}",
    //          _tuple1 == _tuple2);
    
    // Compile-time Error! Type mismatch so these cannot be compared:
    //println!("_struct1 == _struct2 yields: {}",
    //          _struct1 == _struct2);
}

```

#### An example of using Panthom type

When adding two numbers, we need to make sure that those two numbers are in the same type. If we want to implement the add trait for our own type, we also want to do that.

The `Add` trait is defined as 
```rust
// This construction would impose: `Self + RHS = Output`
// where RHS defaults to Self if not specified in the implementation.
pub trait Add<RHS = Self> {
    type Output;

    fn add(self, rhs: RHS) -> Self::Output;
}

// `Output` must be `T<U>` so that `T<U> + T<U> = T<U>`.
impl<U> Add for T<U> {
    type Output = T<U>;
    ...
}
```

but what if we have a struct and want to implement the `Add` trait for it, and at the same time we want to ensure that the type of the two sides match?


```rust
use std::ops::Add;
use std::marker::PhantomData;

/// Create void enumerations to define unit types.
#[derive(Debug, Clone, Copy)]
enum Inch {}
#[derive(Debug, Clone, Copy)]
enum Mm {}

/// `Length` is a type with phantom type parameter `Unit`,
/// and is not generic over the length type (that is `f64`).
///
/// `f64` already implements the `Clone` and `Copy` traits.
#[derive(Debug, Clone, Copy)]
struct Length<Unit>(f64, PhantomData<Unit>);

/// The `Add` trait defines the behavior of the `+` operator.
impl<Unit> Add for Length<Unit> {
     type Output = Length<Unit>;

    // add() returns a new `Length` struct containing the sum.
    fn add(self, rhs: Length<Unit>) -> Length<Unit> {
        // `+` calls the `Add` implementation for `f64`.
        Length(self.0 + rhs.0, PhantomData)
    }
}

fn main() {
    // Specifies `one_foot` to have phantom type parameter `Inch`.
    let one_foot:  Length<Inch> = Length(12.0, PhantomData);
    // `one_meter` has phantom type parameter `Mm`.
    let one_meter: Length<Mm>   = Length(1000.0, PhantomData);

    // `+` calls the `add()` method we implemented for `Length<Unit>`.
    //
    // Since `Length` implements `Copy`, `add()` does not consume
    // `one_foot` and `one_meter` but copies them into `self` and `rhs`.
    let two_feet = one_foot + one_foot;
    let two_meters = one_meter + one_meter;

    // Addition works.
    println!("one foot + one_foot = {:?} in", two_feet.0);
    println!("one meter + one_meter = {:?} mm", two_meters.0);

    // Nonsensical operations fail as they should:
    // Compile-time Error: type mismatch.
    //let one_feter = one_foot + one_meter;
}

```


Adding a field of type `PhantomData<T>` indicates that your type owns data of type `T`. This in turn implies that when your type is dropped, it may drop one or more instances of the type `T`. This has bearing on the Rust compiler’s drop check analysis.

If your struct does not in fact own the data of type `T`, it is better to use a reference type, like `PhantomData<&'a T>` (ideally) or `PhantomData<*const T>` (if no lifetime applies), so as not to indicate ownership.