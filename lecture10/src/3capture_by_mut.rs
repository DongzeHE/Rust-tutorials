fn main() {
    let mut mut_val = String::from("mut");
    let mut fnmut_closure = |VAR| {
        VAR.push_str("-new");
    };

    // cannot borrow immutable because the variable is borrowed as mutable
    // println!("{}", mut_val);

    // cannot borrow mutably second time
    // mut_val.push_str("another_string");


    // ok because closure is already dropped
    println!("{}", mut_val);    
    fnmut_closure(); // END

}
