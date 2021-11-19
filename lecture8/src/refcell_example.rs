use std::cell::RefCell;

fn main() {
    let rc = RefCell::new(5);
    // println!("rc value: {}", *rc); // you cannot deref RefCell directly.
    println!("rc value: {}", *rc.borrow());
    *rc.borrow_mut() = 6;
    println!("rc value: {}", *rc.borrow());
}
