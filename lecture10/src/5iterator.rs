use std::{collections::HashMap, vec};

fn main() {
    let names = vec!["Pascal", "Elvira", "Dominic", "Christoph"];

    for name in &names {
        println!("{}", name);
    }

    println!("{:?}",names);

    let mut hm = HashMap::new();

    hm.insert(1, "a");
    hm.insert(2, "b");

    for kv in hm {
        println!("{}: {}", kv.0, kv.1);
    }

    let v1 = vec![1,2,3,4];
    let v2 = vec![5,6,7,8];
    let sum: i32 = v1.iter()
        .zip(v2.iter())
        .map(|(a, b)| *a * *b)
        .filter(|x| x % 2 == 0)
        .map(|x| x + 10)
        .collect::<Vec<i32>>()
        .iter()
        .sum();
    println!("the result is {}", sum)
    }
