fn main() {
    let mut immut_val = String::from("immut");

    let fn_closure = || {
        println!("Len: {}", immut_val.len()); //immutable ref
            immut_val.push_str("-push");

    };

    println!("Value: {}", immut_val); //ok
{    fn_closure(); //ok
}
    // cannot borrow mutably because it is already borrowed immutably

    // fn_closure();
}
