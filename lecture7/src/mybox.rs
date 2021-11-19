fn main() {
    let x = 5;
    let y = &x; // make reference
    let z = Box::new(x); // copy x into a Box


    assert_eq!(5, x);
    assert_eq!(5, *y); // dereference
    assert_eq!(5, *z); // dereference
    assert_eq!(5, *(z.deref()));
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}