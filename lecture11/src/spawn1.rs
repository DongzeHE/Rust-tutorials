use std::thread;
use std::time::Duration;

// 1. how to use spawn
// 2. use join to wait for child threads
// 2. use move
// 3. move takes ownership

fn main() {
    let v = vec![1,2,3,4];
    // spawned thread
    let handle = thread::spawn(move || {
        for i in &v {
            println!("hi number {} from the spawned thread",i);
        }
    });
    drop(v);


    handle.join().unwrap()
}