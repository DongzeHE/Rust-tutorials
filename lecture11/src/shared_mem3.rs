use std::sync::Mutex;

// what is mutex, interior mutability
// how to define a Mutex
// how to lock
// when will a Mutex be unlocked
// how to share mutex between threads

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    } // unlock

    println!("m = {:?}", m);
}