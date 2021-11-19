fn main() {
    let mut mov_val = String::from("value");
    let fnonce_closure = || {
        let moved_value = mov_val;
    };
    
    fnonce_closure(); // ok
    
    mov_val = String::from("value");

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