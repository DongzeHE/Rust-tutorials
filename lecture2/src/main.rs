fn main() {
        let s_literal = "abc";
        println!("{}", s_literal);
        let s = s_literal.to_string();
        println!("{}", s.chars().nth(1).unwrap()); // ERROR
}
