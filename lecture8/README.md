
# Smart pointers
1. are pointers that point at a chunk of heap memory .
2. own their data.
3. implement `Deref` and `Drop` traits.
4. maintain some metadata.

## Box

Box is the simplist smart pointer, it has no meatadata.

```rust
fn main() {
    let mut b = Box::new(5);
    println!("b = {}", b);
    *b = 6;
    println!("b = {}", b);
}
```

## Reference counting pointer Rc

The data stored in a `Rc` is immutable. The data of `Rc` can have multiple owners. A `Rc` maintains a strong pointer count and a weak pointer count.

#### Strong and weak count

`Rc` maintains two kinds of count, strong pointer count (`Rc::strong_count(&rc)`) and weak pointer count(`Rc::weak_count(&rc)`). 
- strong pointer: A strong pointer can be created by`Rc::clone()` or `Rc::new()`. A strong pointer can access the data strored in a `Rc` directly. If strong pointer count is goes to zero, the data of the `Rc` will be dropped.
- weak pointer: A weak pointer can be generated from `Rc::downgrade(&rc)`. A weak pointer needs to be first upgrade to a strong pointer `weak_ptr.upgrade()` before accessing the data. the `upgrade()` function will return a `Option`. If the inner data of the `Rc` is dropped, i.e., the strong pointer count of the `Rc` is zero, `None` will be returned. 
- If a `Rc` has both strong pointers and weak pointers, the inner data will be dropped if the strong pointer count goes to zero. However, the `Rc` will be dropped if both the strong and weak pointer count are zero. 

```rust
let rc = Rc::new(5);
let strong_ptr1 = Rc::clone(&rc); // second strong pointer
let weak_ptr = Rc::downgrade(&rc); // create a weak pointer
```

# RefCell and interior mutability


## `RefCell<T>`

**Interior mutability is a design pattern in Rust that allows you to mutate data even when there are immutable references to that data**; normally, this action is disallowed by the borrowing rules. To mutate data, the pattern uses **unsafe** code inside a data structure to bend Rust’s usual rules that govern mutation and borrowing.

Recall the **borrowing rules** we learned in previous lectures:
 - At any given time, you can have either one mutable reference or any number of immutable references.
 - References must always be valid.

When using `Box<T>`, the borrowing rules’ invariants are enforced at compile time. **With `RefCell<T>`, these invariants are enforced at runtime.** With references, if you break these rules, you’ll get a compiler error. With `RefCell<T>`, if you break these rules, your program will panic and exit.

Checking borrowing rules at compile time has the advantage of catching errors sooner in the development process, and there is no impact on runtime performance because all the analysis is completed beforehand. For those reasons, checking the borrowing rules at compile time is the best choice in the majority of cases.

However, there are other scenarios where one might want to take advantage of the additional flexibility afforded by checking the borrowing rules at runtime. Because some analyses are impossible, if the Rust compiler can’t be sure the code complies with the ownership rules, it might reject a correct program; in this way, it is conservative. The `RefCell<T>` type is useful when you’re sure your code follows the borrowing rules but the compiler is unable to understand and guarantee that.

```rust
let rc = RefCell::new(5);
// println!("rc value: {}", *rc); // you cannot deref RefCell directly.
println!("rc value: {}", *rc.borrow());
*rc.borrow_mut() = 6;
println!("rc value: {}", *rc.borrow());

```

### When to use each type?
 
 - `Rc<T>` enables multiple owners of the same immutable data; `Box<T>` and `RefCell<T>` have single owners.
 - `Box<T>` allows immutable or mutable borrows checked at compile time; `Rc<T>` allows only immutable borrows checked at compile time; 
 - `RefCell<T>` allows immutable or mutable borrows checked at runtime.

### Borrowing review

Will this code compile? Why?

```rust
fn main() {
    let x = 5;
    let y = &mut x;
    *y = 6;
    let z = &x;
    // *z = 7; ERROR
}
```

Remember that `RefCell<T>` doesn’t get around the borrowing rules completely: the borrow checker in the compiler allows this interior mutability, and the borrowing rules are checked at runtime instead. If you violate the rules, you’ll get a panic! instead of a compiler error.

### Interior mutability example

In this example, we’ll create a library that tracks a value against a maximum value and sends messages based on how close to the maximum value the current value is. This library could be used to keep track of a user’s quota for the number of API calls they’re allowed to make, for example. Our library will only provide the functionality of tracking how close to the maximum a value is and what the messages should be at what times. Applications that use our library will be expected to provide the mechanism for sending the messages: the application could put a message in the application, send an email, send a text message, or something else. The library doesn’t need to know that detail. It only uses a trait we'll provide called Messenger. 

```rust
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}
```

The Messenger trait is the interface our mock object needs to implement so that the mock can be used in the same way a real object is. The other important part is that we want to test the behavior of the set_value method on the LimitTracker. We can change what we pass in for the value parameter, but set_value doesn’t return anything for us to make assertions on. We want to be able to say that if we create a LimitTracker with something that implements the Messenger trait and a particular value for max, when we pass different numbers for value, the messenger is told to send the appropriate messages.

We need a mock object that, instead of sending an email or text message when we call send, will only keep track of the messages it’s told to send. We can create a new instance of the mock object, create a LimitTracker that uses the mock object, call the set_value method on LimitTracker, and then check that the mock object has the messages we expect. The following example shows an attempt to do this, but the borrow checker won't allow it. 

```rust
#[cfg(test)]
mod tests {
    use super::*;

    struct MockMessenger {
        sent_messages: Vec<String>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: vec![],
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.len(), 1);
    }
}
```

What's the problem with this code? Why won't the borrow checker allow this. (Hint: Look at `send()`). When compiling this example, we get the following error:

```
$ cargo test
   Compiling limit-tracker v0.1.0 (file:///projects/limit-tracker)
error[E0596]: cannot borrow `self.sent_messages` as mutable, as it is behind a `&` reference
  --> src/lib.rs:58:13
   |
2  |     fn send(&self, msg: &str);
   |             ----- help: consider changing that to be a mutable reference: `&mut self`
...
58 |             self.sent_messages.push(String::from(message));
   |             ^^^^^^^^^^^^^^^^^^ `self` is a `&` reference, so the data it refers to cannot be borrowed as mutable

error: aborting due to previous error

For more information about this error, try `rustc --explain E0596`.
error: could not compile `limit-tracker`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
```

Modifying the MockMessenger to keep track of the messages, because the send method takes an immutable reference to self. We also can’t take the suggestion from the error text to use &mut self instead, because then the signature of send wouldn’t match the signature in the Messenger trait definition. However, we can fix this example by using interior mutability. We will store the sent_messsages within a `RefCell<T>`, and then the send method will be able to modify hte sent_messages to store the messages we've seen. Here is an example implementation.

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        // --snip--

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
```

The sent_messages field is now of type `RefCell<Vec<String>>` instead of `Vec<String>`. In the new function, we create a new `RefCell<Vec<String>>` instance around the empty vector. 

For the implementation of the send method, the first parameter is still an immutable borrow of self, which matches the trait definition. We call borrow_mut on the `RefCell<Vec<String>>` in self.sent_messages to get a mutable reference to the value inside the `RefCell<Vec<String>>`, which is the vector. Then we can call push on the mutable reference to the vector to keep track of the messages sent during the test.

The last change we have to make is in the assertion: to see how many items are in the inner vector, we call borrow on the `RefCell<Vec<String>>` to get an immutable reference to the vector.

---

### How `Refcell<T>` works

When creating immutable and mutable references, we use the `&` and `&mut` syntax, respectively. With `RefCell<T>`, we use the `borrow` and `borrow_mut` methods, which are part of the safe API that belongs to `RefCell<T>`. The `borrow` method returns the smart pointer type `Ref<T>`, and `borrow_mut` returns the smart pointer type `RefMut<T>`. Both types implement `Deref`, so we can treat them like regular references.
 
The `RefCell<T>` keeps track of how many `Ref<T>` and `RefMut<T>` smart pointers are currently active. Every time we call borrow, the `RefCell<T>` increases its count of how many immutable borrows are active. When a `Ref<T>` value goes out of scope, the count of immutable borrows goes down by one. Just like the compile-time borrowing rules, `RefCell<T>` **lets us have many immutable borrows or one mutable borrow at any point in time.**
 
Now if we violate the borrowing rules, we'll get an error at compile time rather than at runtime. Here is an example.

```rust
    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            let mut one_borrow = self.sent_messages.borrow_mut();
            let mut two_borrow = self.sent_messages.borrow_mut();

            one_borrow.push(String::from(message));
            two_borrow.push(String::from(message));
        }
    }
```

We can see that with refcell this example will compile, but when we run it the program will panic.

```
thread 'main' panicked at 'already borrowed: BorrowMutError', src/lib.rs:60:53
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Notice that the code panicked with the message already borrowed: BorrowMutError. This is how `RefCell<T>` handles violations of the borrowing rules at runtime.

### Combining Rc and RefCell

It is common to use `RefCell<T>` together with `Rc<T>`. Recall that `Rc<T>` lets you have multiple owners of some data, but it only gives immutable access to that data. **If you have an `Rc<T>` that holds a `RefCell<T>`, you can get a value that can have multiple owners and that you can mutate!** Because `Rc<T>` holds only immutable values, we can’t change any of the values in the list once we’ve created them. Now we will add in `RefCell<T>` to gain the ability to change the values.

```rust
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
```

Here we create a value that is an instance of `Rc<RefCell<i32>>` and store it in a variable named value so we can access it directly later. Then we create a List in a with a Cons variant that holds value. We need to clone value so both a and value have ownership of the inner 5 value rather than transferring ownership from value to a or having a borrow from value.

We wrap the list a in an `Rc<T>` so when we create lists b and c, they can both refer to a. After we’ve created the lists in a, b, and c, we add 10 to the value in value. We do this by calling borrow_mut on value, which uses automatic dereferencing to dereference the `Rc<T>` to the inner `RefCell<T>` value. The borrow_mut method returns a `RefMut<T>` smart pointer, and we use the dereference operator on it to change the inner value. When we print a, b, and c, we can see that they all have the modified value of 15 rather than 5.

```
$ cargo run
   Compiling cons-list v0.1.0 (file:///projects/cons-list)
    Finished dev [unoptimized + debuginfo] target(s) in 0.63s
     Running `target/debug/cons-list`
a after = Cons(RefCell { value: 15 }, Nil)
b after = Cons(RefCell { value: 3 }, Cons(RefCell { value: 15 }, Nil))
c after = Cons(RefCell { value: 4 }, Cons(RefCell { value: 15 }, Nil))
```

Here we have an outwardly immutable List value. But we can use the methods on `RefCell<T>` that provide access to its interior mutability so we can modify our data when we need to. The runtime checks of the borrowing rules protect us from data races, and it’s sometimes worth trading a bit of speed for this flexibility in our data structures.
 
The standard library has other types that provide interior mutability, such as `Cell<T>`, which is similar except that instead of giving references to the inner value, the value is copied in and out of the `Cell<T>`. There’s also `Mutex<T>`, which offers interior mutability that’s safe to use across threads.

### Memory leaks

Rust’s memory safety guarantees make it difficult, but not impossible, to accidentally create memory that is never cleaned up (known as a memory leak). Preventing memory leaks entirely is not one of Rust’s guarantees in the same way that disallowing data races at compile time is.

We can see that Rust allows memory leaks by using `Rc<T>` and `RefCell<T>`: it’s possible to create references where items refer to each other in a cycle. This creates memory leaks because the reference count of each item in the cycle will never reach 0, and the values will never be dropped.

We can see this in the following example. 

```rust
use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // What will happen?
    // println!("a next item = {:?}", a.tail());
}
```

If we uncomment the println, what will happen? We will overflow the stack. The reference count of the `Rc<List>` instances in both a and b are 2 after we change the list in a to point to b. At the end of main, Rust drops the variable b, which decreases the reference count of the `Rc<List>` instance from 2 to 1. The memory that `Rc<List>` has on the heap won’t be dropped at this point, because its reference count is 1, not 0. Then Rust drops a, which decreases the reference count of the a `Rc<List>` instance from 2 to 1 as well. This can’t be dropped either, because the other `Rc<List>` instance still refers to it. 
 
This is an example of a reference cycle, which will result in leaked memory which will remain uncollected as long as the program is running. 

### Preventing reference cycles

You can also create a weak reference to the value within an `Rc<T>` instance by calling Rc::downgrade and passing a reference to the `Rc<T>`. When you call Rc::downgrade, you get a smart pointer of type `Weak<T>`. Instead of increasing the strong_count in the `Rc<T>` instance by 1, calling Rc::downgrade increases the weak_count by 1. The `Rc<T>` type uses weak_count to keep track of how many `Weak<T>` references exist, similar to strong_count. The difference is the weak_count doesn’t need to be 0 for the `Rc<T>` instance to be cleaned up.

Ownership is expressed using the strong references, while weak references don't express an ownership relation. 

Because the value that `Weak<T>` references might have been dropped, to do anything with the value that a `Weak<T>` is pointing to, you must make sure the value still exists. Do this by calling the upgrade method on a `Weak<T>` instance, which will return an `Option<Rc<T>>`.

We'll work through an example of using weak references using a tree data structure. First we'll create a Node struct.

```rust
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
}
```

We want a Node to own its children, and we want to share that ownership with variables so we can access each Node in the tree directly. To do this, we define the `Vec<T>` items to be values of type `Rc<Node>`. We also want to modify which nodes are children of another node, so we have a `RefCell<T>` in children around the `Vec<Rc<Node>>`.

Next, we’ll use our struct definition and create one Node instance named leaf with the value 3 and no children, and another instance named branch with the value 5 and leaf as one of its children. We clone the `Rc<Node>` in leaf and store that in branch, meaning the Node in leaf now has two owners: leaf and branch. We can get from branch to leaf through branch children, but there’s no way to get from leaf to branch. The reason is that leaf has no reference to branch and doesn’t know they’re related. We want leaf to know that branch is its parent.

```rust
fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
}
```

To make the child node aware of its parent, we need to add a parent field to our Node struct definition. The trouble is in deciding what the type of parent should be. We know it can’t contain an `Rc<T>`, because that would create a reference cycle with leaf.parent pointing to branch and branch.children pointing to leaf, which would cause their strong_count values to never be 0.
 
So instead of `Rc<T>`, we’ll make the type of parent use `Weak<T>`, specifically a `RefCell<Weak<Node>>`. 
 
The reason for this is that we want the parent to own its children, so if a parent is dropped the children should be as well. However, we don't want the parent to drop if the child node still exists. This is where we may use weak references. 

```rust
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}
```

This enables a node to refer to the parent without owning the parent. We can use this in the example shown below.

```rust
fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}
```

When running this example, we can see we don't get a stack overflow like in this previous one. This is because we didn't create a reference cycle. We can visualize the strong and weak count references also, which is shown below.

```rust
fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
```


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
