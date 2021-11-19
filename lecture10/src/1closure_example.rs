fn main() {
    let intro = String::from("name\tage");
    let print_user_age = |name, age| println!("{}\n{}\t{}\n", intro, name, age);

    for (name, age) in [
        (String::from("Alice"), 5),
        (String::from("Bob"), 7),
        (String::from("Mallory"), 20),
    ]
    .iter()
    {
        print_user_age(name, age);
    }
}
