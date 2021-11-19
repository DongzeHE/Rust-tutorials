# 09/17
In today's lecture, we will talk about 
1. struct/impl
2. lifetime


## Structs

A struct, or structure, is a custom data type that lets you name and package together multiple related values that make up a meaningful group. *Struct* allows you to group multiple pieces of data with different types together. You can name each piece of data and query them by name. Each piece of data and its name is called a field. More at [TRPL](https://doc.rust-lang.org/book/ch05-00-structs.html) and [Rust By Example](https://doc.rust-lang.org/rust-by-example/custom_types/structs.html).

### Defining and Instantiating Structs


#### Defining a Struct

A structure (struct) can be defined by the keyword `struct`, followed by the name of the `Struct`, and then the fields in the Struct.

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

#### Instantiate a struct

To instantiate a struct, use the struct name followed by curly brankets and specify the value for each field.

```rust
let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};
```

#### Mutable instance

`struct` can be mutable, but making a single field mutable is not allowed.

```rust
let mut user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};

user1.email = String::from("anotheremail@example.com");
```

#### Field Init Shorthand

When the name of a field is the same as a valid variable, we can use the *field init shorthand* syntax to remove some repetition.

```rust
let email = String::from("someone@example.com");
let username = String::from("someusername123");

let user = User {
    email,
    username,
    active: true,
    sign_in_count: 1,
};
```

#### Struct Update Syntax

Using struct update syntax, you can use one instance to help to create another instance of a Struct.

```rust
let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};

// The following two commands are same
// with struct update syntax
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1,
};

// without struct update syntax
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    active: user1.active,
    sign_in_count: user1.sign_in_count,
};
```

#### Tuple Structs

Structs can be very similar to tuples, called *tuple structs*.

```rust
struct Color(i32, i32, i32);

let black = Color(0, 0, 0);
```

You can destructure the fields into their individual pieces, you can use a `.` followed by the index to access an individual value.


#### Adding derived traits 

Rust provides many derived traits for us. These traits can be plug-and-use. Some of them are very useful for developing your program, such as `Debug` trait. To use these derived traits, you need to add the annotation just before the struct definition. For example, `#[derive(Debug)]`. This trait lets Rust know how to print your Structs.

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);
}
```

Putting the specifier `:?` inside the curly brackets tells `println!` we want to use `Debug` output format.  If your struct is very complex, you can use `:#?` to make the output nicer.


### Defining methods for Structs

In OOP, sometimes you want to assign some specific methods that an instance of a struct can call. In Rust, this is called *methods* of structs.

To implement methods for structs, we need to use the `impl` keyword. One struct can have multiple `impl` blocks.

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn double(&mut self) {
        *self.width = self.width * 2;
        *self.height = self.height * 2;
    }
    fn take_ownership(self) {
    }

}

fn main() {
    let mut rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    rect1::fn_1(5);
    let rect2 = Rectangle {
        width: 10,
        height: 60,
    };
    
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    
    rect1.double();

    println!("Can doubled rect1 hold rect2? {}", rect1.can_hold(&rect2));
    
}
```

In the signature for the `area()`, we use `&self` instead of rectangle: `&Rectangle` because Rust knows the type of `self` is `Rectangle` due to this method’s being inside the `impl Rectangle` context. Note that we still need to use the `&` before `self`, just as we did in `&Rectangle`. Methods can take ownership of self, borrow `self` immutably as we’ve done here, or borrow `self` mutably, just as they can any other parameter.

Sometimes we want to define functions for structs. The difference between a *method* and a *function* of a struct is that a *function* of a struct doesn't need an instance of the struct to work with.

```rust
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

```

To call this associated function, we use the `::` syntax with the struct name; `let sq = Rectangle::square(3);` is an example. This function is namespaced by the struct: the `::` syntax is used for both associated functions and namespaces created by modules. 

#### Taking reference as fields

Structs can store references to data owned by something else, but to do so requires the use of *lifetimes*.

Lifetimes ensure that the data referenced by a struct is valid for as long as the struct is. Let’s say you try to store a reference in a struct without specifying lifetimes, like this, which **won’t work**:

```rust
struct User {
    username: &str,
    email: &str,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: "someone@example.com",
        username: "someusername123",
        active: true,
        sign_in_count: 1,
    };
}
```

The compiler will complain that it needs lifetime specifiers.


## Lifetimes
The lifetime of a variable starts at the time when the variable is defined and ends after the last time that the variable is used.

### The borrow checker

```rust
{
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
}                         // ---------+
```

### Generic Lifetimes in Functions
Lifetime of a reference is not always explicit. For example:

```rust
fn longest(x:&str, y:&str) -> &str {
    if x.len() > y.len() { x } else { y}
} // this funcion doesn't work
```
When running this function, Rust doesn't know the lifetime of `x` and `y`. The following situation may happen:
```rust
{ 
    let x = String::from(“hi”);
    let z;
    { let y = String::from(“there”);
        z = longest(&x,&y); //will be &y
    } //drop y, and thereby z
    println!(“z = {}”,z);//yikes!
}
```

To fix it, we need to explicitly specify that  `x` and `y` must have the same lifetime, and the returned reference shares it. This can be done using the apostrophe `'` followed by a lowercase, like generic types. By convention, we use `'a`.

```rust
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
```

So, the problem of the previous `longest()` definition is that that function may return `x` or `y`, so Rust cannot tell what's the exact lifetime of the return value. To solve this, we need to tell Rust explicitly that `x` and `y` need to have the same lifetime, or we will return the one with the shorter lifetime. Lifetimes on function or method parameters are called *input lifetimes*, and lifetimes on return values are called *output lifetimes*.

```rust
fn longest<'a>(x:&'a str, y:&'a str) -> &'a str {
  if x.len() > y.len() { x } else { y }
}

```

Note:
- Each reference to a value of type `t` has a `lifetime parameter`. 
    - `&t` (and `&mut t`) – lifetime is implicit
    - `&'a t` (and `&'a mut t`) – lifetime `'a` is explicit
- Where do the lifetime names come from?
    - When left implicit, they are generated by the compiler
    - Global variables have lifetime `'static`, which are encoded in the binary of the program.

### Lifetimes FAQ
- How does the Rust compiler figure out *lifetimes*?
    - The first rule is that each parameter that is a reference gets its lifetime parameter. In other words, a function with one parameter gets one lifetime parameter: `fn foo<'a>(x: &'a i32)`; a function with two parameters gets two separate lifetime parameters: `fn foo<'a, 'b>(x: &'a i32, y: &'b i32)`; and so on.
    - The second rule is if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters: `fn foo<'a>(x: &'a i32) -> &'a i32`.
    - The third rule is if there are multiple input lifetime parameters, but one of them is `&self` or `&mut self` because this is a method, the lifetime of `self` is assigned to all output lifetime parameters. This third rule makes methods much nicer to read and write because fewer symbols are necessary.
- When do we use explicit lifetimes?
    - When more than one var/type needs the same lifetime (like the `longest` function)

- How do I tell the compiler exactly which lines of code lifetime `'a` covers?
    - You can't. The compiler will (always) figure it out

- How does lifetime subsumption work?
    - If lifetime `'a` is longer than `'b`, we can use `'a` where `'b` is expected; can require this with `'b: 'a`.
        - Permits us to call `longest(&x,&y)` when `x` and `y` have different lifetimes, but one outlives the other.
        ```rust
        fn longest<'b, 'a: 'b>(x:&'a str, y:&'b str) -> &'b str {
            if x.len() > y.len() { x } else { y}
        } // this funcion doesn't work
        ```
- Can we use lifetimes in data definitions?
    - Yes; we will see this later when we define structs, enums, etc.


### [Non Lexical Lifetimes(NLL)](http://blog.pnkfx.org/blog/2019/06/26/breaking-news-non-lexical-lifetimes-arrives-for-everyone)
Rust has been updated to support NLL -- lifetimes that end before the surrounding scope:
```rust
fn main() {                               // SCOPE TREE
                                          //
    let mut names =                       // +- `names` scope start
        ["abe", "beth", "cory", "diane"]; // |
                                          // |
    let alias = &mut names[0];            // | +- `alias` scope start
                                          // | |
    *alias = "alex"; // <------------------------ write to `*alias`
                                          // | |
    println!("{}", names[0]); // <--------------- read of `names[0]`
                                          // | |
                                          // | +- `alias` scope end
                                          // +- `name` scope end
}

```

In practice, this is important when


```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}

```

So how do we use references in struct definition? 

```rust
struct User<'a> {
    username: &'a str,
    email: &'a str,
    sign_in_count: u64,
    active: bool,
}

#![allow(unused)]
fn main() {
    let user1 = User {
        email: "someone@example.com",
        username: "someusername123",
        active: true,
        sign_in_count: 1,
    };
}
```

## Quiz with solution


## Quiz

### Heap v.s. Stack

1. Will the following code compile? why? If not, how to fix it? 
    ```rust
    fn main() { 
        let x = String::from("hello");
        let y = x;
        println!("{}, world!", y);
        println!("{}, world!", x); 
    }
    ```

    - No, becuase it called `x` after moving its value to `y`. To fix this, we can define `y` as `let y = x.clone()`   


2. Will the following code compile? why? If not, how to fix it? 
    ```rust
    let x = 5;
    let y = x;
    println!("{} = 5!", y
    println!("{} = 5!", x); 
    ```

    - Yes, it can compile. This is because primitive types implement `Copy` trait, so their value will be copied when assigning.

### Ownership Rules


3. Owner of str’s data at HERE ?

    ```rust
    fn foo(str:String) -> usize {
        let x = str; 
        let y = &x;
        let z = x;
        let w = &y;
        // HERE
    }
    ```
* x
* y
* z
* w
    - z, because x is assigned to z.

4. in question #3, when will the value of `x` be dropped? 

    - when the scope defined by the curly brackets ends.

### String reference v.s. String Slice

5. If we have a string `let s = String::from("hello")`, What are the differences (structure, layer of indirection) between `&s` and `&s[..]`?

    - `&s` is an immutable reference. it consists of a pointer to the *string variable*, which locates on the stack and has a pointer to the actual value in the heap. `&s[..]` is a string slice. It consists of a pointer and a length, and its pointer points directly to the starting position of the actual value in the heap. 

### Ownership Transfer in Function Calls

6. Will the following code compile? why? If not, how to fix it? 

    ```rust
    fn main() {
        let s1 = String::from(“hello”);
        let s2 = id(s1);
        println!(“{}”,s2);
        println!(“{}”,s1);
    }

    fn id(s:String) -> String {
        s
    }
    ```

- No, because the function took ownership of the string, so we cannot access it after evaluating the function. To fix this, the function should take a reference as the input.

    ```rust
    fn id(s:&String) -> String {
        s.to_string()
    }
    ```

### Borrowing

7. What does this evaluate to?

```rust
{ let mut s1 = String::from(“Hello!“);
    {
        let s2 = &s1;
        s2.push_str(“World!“);
        println!(“{}“, s2)    
    }  
}

```

**A.** "Hello!"\
**B.** "Hello! World!"\
**C.** Error\
**D.** "Hello!World!"

- C. `s2` is an immutable reference, we cannot use it to modify the value.

### Mutability

8. What is printed?

```rust
fn foo(s: &mut String) -> usize {
    s.push_str("Bob");
    s.len()
}
fn main() {
    let mut s1 = String::from("Alice");
    println!("{}",foo(&mut s1));
}
```

**A.** 0\
**B.** 8\
**C.** Error\
**D.** 5

- B. The string is modified using `s1` as "AliceBob". So the total length is $8$.

9. What's wrong here? 
```rust
let mut s1 = String::from(“hello”);
{ 
    let s2 = &s1;
    let s3 = &s1;
    let s4 = &mut s1;
    let s5 = &mut s1;
    println!(”String is {}”,s1);
    println!(”String is {}”,s2);
    println!(”String is {}”,s3);
    println!(”String is {}”,s4);

}
s1.push_str(“ there”);
println!(”String is {}”,s1);

```

- We can create multiple immutable references **OR** one mutable reference.

