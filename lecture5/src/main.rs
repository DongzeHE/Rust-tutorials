enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {

    // Instantiate enum
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    // pattern matching
    let v6 = match home {
        IpAddr::V4(127,b,c,d) => {
            let v4 = format!("127.{}.{}.{}", b,c,d);
            println!("I am a V4 starting with 127: {}", &v4);
            None
        },
        IpAddr::V4(a,b,c,d) => {
            let v4 = format!("{}.{}.{}.{}", a, b, c, d);
            println!("I am also a V4 but not start with 127: {}", &v4);
            None
        },
        _ => Some(home),
        // All possible cases have to be handled.
    };

    if let Some(_) = v6 {
        println!("hurray!")
    }

    if let IpAddr::V6(s) = loopback {
        println!("IpV6: {}",s)
    }
    match loopback {
        IpAddr::V4(127, b, c, d) | IpAddr::V4(128, b, c, d) => println!("V4 127.x.x.x or 128.x.x.x"),
        IpAddr::V4(127, b, c, d) if d == 1 => {
            println!("V4 127.x.x.1")
        },
        IpAddr::V4(a, b, c, d) => println!("Is V4"),
        IpAddr::V6(s) => match s.as_str() {
            "::1" => println!("V6,::1"),
            _ => {},            
        }
    }
}


