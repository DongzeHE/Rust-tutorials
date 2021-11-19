use std::sync::mpsc;
use std::thread;
use std::time::Duration;

// 1. why message passing? 
//   - Talk with subthreads
//   - shared memory is hard to handle.

// how to create a channel
// how to send and receive values
// what's the return type of send and recv
// how to send multiple values
// The difference between recv and try_recv (recv blocks the main thread while try_recv not)



fn main() {
    // multiple producer, single consumer. 
    // (transmitter, receiver)
    let (tx, rx) = mpsc::channel();
    
    // clone the transmitter so that each child thread gets one
    let tx1 = tx.clone();

    thread::spawn(move || {
        // let vals = vec![
        //     String::from("one rubber duck in river 1"),
        //     String::from("two rubber ducks in river 1"),
        //     String::from("three rubber ducks in river 1"),
        //     String::from("four rubber ducks in river 1"),
        // ];
        let s = String::from("one rubber duck in river 1");

        // putting a rubber duck in the river upstream
        // for val in vals {
            // send() returns a Result
            tx.send(s).unwrap();
            thread::sleep(Duration::from_secs(1));
        // }
        // send takes the ownership of val
        // println!("{}",val); ERROR!
    });
    
    //     thread::spawn(move || {
    //     let vals = vec![
    //         String::from("one rubber duck in river 2"),
    //         String::from("two rubber ducks in river 2"),
    //         String::from("three rubber ducks in river 2"),
    //         String::from("four rubber ducks in river 2"),
    //     ];
    //     // putting a rubber duck in the river upstream
    //     for val in vals {
    //         tx1.send(val).unwrap();
    //         thread::sleep(Duration::from_secs(1));
    //     }
    //     // println!("{}",val); ERROR!
    // });
    // // rx blocks the main thread!
    // // it implements the Iterator trait
    // let a = rx;
    // for received in rx {
    //     println!("Got: {}", received);
    // }
    // let received = rx.recv().unwrap();
    let received = 
    println!("Got: {}", received);

}
