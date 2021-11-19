fn main() {
    let mut b = Box::new(5);
    // b = 6;
    // b = Box::new(6);
    *b = 6;
    println!("b = {}", b);
}