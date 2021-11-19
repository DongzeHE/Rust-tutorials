use std::{cell::RefCell, process::Command, rc::Rc, borrow::BorrowMut};

fn main() {
    // let output = Command::new("ls")
    //     .arg("-l")
    //     .arg("-h")
    //     .output()
    //     .expect("fail to execute process.");
    // println!("{}", String::from_utf8(output.stdout).unwrap());

    let a = Rc::new(RefCell::new(5));
    let b = RefCell::borrow_mut(&a);
}


